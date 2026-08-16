//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 748/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk748(t5638: f64, t822: f64, t9419: f64, t28023: f64, t7290: f64, t1890: f64, t28013: f64, t28236: f64, t739: f64, t10036: f64, t2021: f64, t1980: f64, t9816: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28856 = t822 * t5638 * t9419;
    let t28924 = t7290 * t28023;
    let t28953 = t1890 * t28013;
    let t28957 = t739 * t28236;
    let t28973 = t2021 * t10036;
    let t28983 = t1980 * t9816;
    (t28856, t28924, t28953, t28957, t28973, t28983)
}
