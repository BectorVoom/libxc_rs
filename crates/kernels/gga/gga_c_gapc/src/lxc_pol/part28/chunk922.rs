//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 922/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk922<F: Float>(t11250: F, t11255: F, t11259: F, t11265: F, t11268: F, t11274: F, t11276: F, t11237: F, t11252: F, t11263: F, t12025: F, t12026: F, t12027: F, t12023: F, t209: F, t3804: F, t575: F) -> (F, F, F) {
    let t12028 = 0.32188557442181220751e-6 * t11250;
    let t12030 = 0.86898242813537603825e-4 * t11255;
    let t12031 = 0.86898242813537603825e-4 * t11259;
    let t12033 = 0.22776267492663374278e-4 * t11265;
    let t12034 = 0.2530696388073708253e-5 * t11268;
    let t12035 = 0.73811977985483157379e-6 * t11274;
    let t12036 = 0.12147342662753799615e-3 * t11276;
    let t12037 = -0.54311401758461002391e-5 * t11237 - t12025 + t12026 - t12027 + t12028 - 0.5974254193430710263e-4 * t11252 + t12030 + t12031 - 0.54311401758461002391e-5 * t11263 + t12033 - t12034 - t12035 + t12036;
    let t12038 = t12023 + t12037;
    let t12039 = t12038 * t209;
    let t12040 = t3804 * t575;
    (t12038, t12039, t12040)
}
