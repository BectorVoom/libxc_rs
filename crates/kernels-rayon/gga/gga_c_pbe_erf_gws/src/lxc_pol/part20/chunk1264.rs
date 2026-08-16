//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1264/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1264(t13939: f64, t3083: f64, t13953: f64, t3070: f64, t1192: f64, t26654: f64, t829: f64, t830: f64, t13808: f64, t14584: f64, t4130: f64, t51650: f64) -> (f64, f64, f64, f64, f64) {
    let t54667 = 7.0_f64 / 144.0_f64 * t3083 * t13939;
    let t54681 = t13953 * t3070;
    let t54682 = 7.0_f64 / 72.0_f64 * t54681;
    let t54709 = t26654 * t1192;
    let t54711 = t829 * t830 * t54709;
    let t54716 = t13808 * t14584;
    let t54717 = 7.0_f64 / 1152.0_f64 * t54716;
    let t54719 = t51650 * t4130;
    (t54667, t54682, t54711, t54717, t54719)
}
