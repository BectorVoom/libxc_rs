//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1041/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1041<F: Float>(t11240: F, t11244: F, t11246: F, t11250: F, t11255: F, t11259: F, t11265: F, t11268: F, t11274: F, t11276: F, t11237: F, t11252: F, t11263: F) -> F {
    let t12025 = F::new(0.1545050757224698596e-4) * t11240;
    let t12026 = F::new(0.84356546269123608433e-6) * t11244;
    let t12027 = F::new(0.52638484871933131665e-3) * t11246;
    let t12028 = F::new(0.32188557442181220751e-6) * t11250;
    let t12030 = F::new(0.86898242813537603825e-4) * t11255;
    let t12031 = F::new(0.86898242813537603825e-4) * t11259;
    let t12033 = F::new(0.22776267492663374278e-4) * t11265;
    let t12034 = F::new(0.2530696388073708253e-5) * t11268;
    let t12035 = F::new(0.73811977985483157379e-6) * t11274;
    let t12036 = F::new(0.12147342662753799615e-3) * t11276;
    let t12037 = -F::new(0.54311401758461002391e-5) * t11237 - t12025 + t12026 - t12027 + t12028 - F::new(0.5974254193430710263e-4) * t11252 + t12030 + t12031 - F::new(0.54311401758461002391e-5) * t11263 + t12033 - t12034 - t12035 + t12036;
    t12037
}
