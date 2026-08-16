//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 482/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk482(t3242: f64, t5392: f64, t3240: f64, t123: f64, t3247: f64, t1088: f64, t1089: f64, t5398: f64, t3237: f64, t4721: f64, t423: f64, t1671: f64, t4740: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5971 = t3242 * t5392;
    let t5972 = t3240 * t5971;
    let t5973 = t123 * t5972;
    let t5975 = t3247 * t5392;
    let t5976 = t1088 * t5975;
    let t5977 = t123 * t5976;
    let t5979 = t1089 * t5398;
    let t5980 = t1088 * t5979;
    let t5981 = t123 * t5980;
    let t5983 = t3237 - 0.11872222222222222222e-1_f64 * t4721 - 0.11872222222222222222e-1_f64 * t5973 + 0.35616666666666666666e-1_f64 * t5977 + 0.17808333333333333333e-1_f64 * t5981;
    let t5985 = 0.621814e-1_f64 * t5983 * t423;
    let t5987 = 2.0_f64 * t4740 * t1671;
    (t5971, t5973, t5975, t5977, t5979, t5981, t5985, t5987)
}
