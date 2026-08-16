//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1044/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1044(t11240: f64, t11244: f64, t11246: f64, t11250: f64, t11255: f64, t11259: f64, t11265: f64, t11268: f64, t11274: f64, t11276: f64, t11237: f64, t11252: f64, t11263: f64) -> f64 {
    let t12025 = 0.1545050757224698596e-4_f64 * t11240;
    let t12026 = 0.84356546269123608433e-6_f64 * t11244;
    let t12027 = 0.52638484871933131665e-3_f64 * t11246;
    let t12028 = 0.32188557442181220751e-6_f64 * t11250;
    let t12030 = 0.86898242813537603825e-4_f64 * t11255;
    let t12031 = 0.86898242813537603825e-4_f64 * t11259;
    let t12033 = 0.22776267492663374278e-4_f64 * t11265;
    let t12034 = 0.2530696388073708253e-5_f64 * t11268;
    let t12035 = 0.73811977985483157379e-6_f64 * t11274;
    let t12036 = 0.12147342662753799615e-3_f64 * t11276;
    let t12037 = -0.54311401758461002391e-5_f64 * t11237 - t12025 + t12026 - t12027 + t12028 - 0.5974254193430710263e-4_f64 * t11252 + t12030 + t12031 - 0.54311401758461002391e-5_f64 * t11263 + t12033 - t12034 - t12035 + t12036;
    t12037
}
