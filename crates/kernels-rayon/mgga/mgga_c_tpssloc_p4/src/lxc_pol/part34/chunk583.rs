//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 583/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk583(t3297: f64, t5971: f64, t136: f64, t1113: f64, t5975: f64, t5979: f64, t3282: f64, t3294: f64, t4721: f64, t4770: f64, t5973: f64, t5977: f64, t5981: f64, t5993: f64, t6000: f64, t6006: f64, t6008: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6011 = t3297 * t5971;
    let t6012 = t136 * t6011;
    let t6014 = t1113 * t5975;
    let t6015 = t136 * t6014;
    let t6017 = t1113 * t5979;
    let t6018 = t136 * t6017;
    let t6020 = -0.9494625e0_f64 * t5993 + 0.1898925e1_f64 * t6000 + t3282 - 0.19931111111111111111e0_f64 * t4721 - 0.19931111111111111111e0_f64 * t5973 + 0.59793333333333333334e0_f64 * t5977 + 0.29896666666666666667e0_f64 * t5981 + 0.15358125e0_f64 * t6006 + 0.3071625e0_f64 * t6008 + t3294 - 0.10954222222222222222e0_f64 * t4770 - 0.27385555555555555556e-1_f64 * t6012 + 0.16431333333333333333e0_f64 * t6015 + 0.82156666666666666667e-1_f64 * t6018;
    (t6011, t6012, t6014, t6015, t6017, t6018, t6020)
}
