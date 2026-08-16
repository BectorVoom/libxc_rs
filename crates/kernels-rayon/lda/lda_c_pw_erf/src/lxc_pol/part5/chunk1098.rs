//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1098/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1098(t156: f64, t7914: f64, t426: f64, t7919: f64, t14685: f64, t14718: f64, t127: f64, t14684: f64, t14692: f64, t14844: f64, t14850: f64, t1832: f64, t1852: f64, t2610: f64, t5578: f64, t6121: f64, t7116: f64) -> (f64, f64, f64, f64, f64) {
    let t20427 = t156 * t7914;
    let t20428 = t426 * t20427;
    let t20430 = t156 * t7919;
    let t20431 = t426 * t20430;
    let t20433 = 3.8973666666666666_f64 * t14685;
    let t20434 = 4.5469277777777775_f64 * t14718;
    let t20435 = -88.1424_f64 * t127 * t7116 * t1832 + 17.62848_f64 * t127 * t5578 * t2610 + 17.62848_f64 * t127 * t1852 * t6121 + t20428 / 6.0_f64 + 2.0_f64 * t20431 - t14684 - t20433 + t14692 - t14844 - t14850 + t20434;
    (t20427, t20430, t20433, t20434, t20435)
}
