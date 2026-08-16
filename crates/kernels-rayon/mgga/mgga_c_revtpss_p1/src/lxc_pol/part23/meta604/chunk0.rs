//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2257/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2257(t24129: f64, t24176: f64, t1079: f64, t1076: f64, t11201: f64, t16284: f64, t1652: f64, t1680: f64, t1696: f64, t20175: f64, t20191: f64, t23959: f64, t24044: f64, t24048: f64, t24061: f64, t24068: f64, t3058: f64, t342: f64, t386: f64, t4747: f64, t4752: f64, t4935: f64, t6235: f64, t6245: f64, t6251: f64, t6259: f64, t6351: f64, t6393: f64) -> (f64, f64, f64) {
    let t24177 = t24129 + t24176;
    let t24178 = t1079 * t24177;
    let t24185 = 0.65854491829355115987e0_f64 * t342 * t24044 - 0.39512695097613069591e1_f64 * t1076 * t24048 - 0.19756347548806534796e1_f64 * t4752 * t6393 + 0.39512695097613069591e1_f64 * t4747 * t6251 + 0.39512695097613069591e1_f64 * t4752 * t6351 + 0.65854491829355115987e0_f64 * t23959 * t386 + 0.19756347548806534796e1_f64 * t6235 * t1680 + 0.39512695097613069591e1_f64 * t3058 * t24061 - 0.19756347548806534796e1_f64 * t4935 * t6393 + 0.39512695097613069591e1_f64 * t16284 * t6245 - 0.39512695097613069591e1_f64 * t11201 * t24068 - 0.19756347548806534796e1_f64 * t4747 * t6259 - 0.65854491829355115987e0_f64 * t1076 * t24178 - 0.39512695097613069591e1_f64 * t20191 * t1652 - 0.39512695097613069591e1_f64 * t20175 * t1696;
    (t24177, t24178, t24185)
}
