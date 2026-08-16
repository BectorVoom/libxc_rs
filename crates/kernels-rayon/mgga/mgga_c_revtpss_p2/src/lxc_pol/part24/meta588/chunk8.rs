//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1844/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1844(t6922: f64, t1868: f64, t1907: f64, t198: f64, t21937: f64, t39989: f64, t4139: f64, t4147: f64, t47084: f64, t47086: f64, t532: f64, t5536: f64, t5541: f64, t6781: f64, t6816: f64, t73499: f64, t86819: f64, t86825: f64, t86828: f64, t91984: f64, t91985: f64, t92013: f64, t92014: f64, t92015: f64, t92016: f64) -> f64 {
    let t92482 = t6922 * t6922;
    let t92490 = -3.0_f64 * t198 * t4147 * t532 * t92482 + 24.0_f64 * t1868 * t4139 * t86828 + 24.0_f64 * t1868 * t5536 * t86819 - 4.0_f64 * t1907 * t5541 * t86825 + 18.0_f64 * t21937 * t4139 * t6816 + 12.0_f64 * t5541 * t6781 * t73499 - t39989 - t47084 - t47086 - t91984 - t91985 + t92013 - t92014 + t92015 + t92016;
    t92490
}
