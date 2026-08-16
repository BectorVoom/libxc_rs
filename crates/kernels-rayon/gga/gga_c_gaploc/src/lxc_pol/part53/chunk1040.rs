//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1040/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1040(t209: f64, t50939: f64, t50949: f64, t50953: f64, t50958: f64, t50962: f64, t50966: f64, t50977: f64, t50979: f64, t47107: f64, t47114: f64, t47120: f64) -> (f64, f64, f64, f64) {
    let t50983 = (t50939 + t50949 + t50953 + t50958 + t50962 + t50966 + t50977 + t50979) * t209;
    let t50984 = 4.0_f64 * t47107;
    let t50985 = 4.0_f64 * t47114;
    let t50986 = 4.0_f64 * t47120;
    (t50983, t50984, t50985, t50986)
}
