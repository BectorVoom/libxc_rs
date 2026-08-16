//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1048/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1048(t102344: f64, t117084: f64, t121210: f64, t1388: f64, t1393: f64, t2036: f64, t2040: f64, t2075: f64, t2096: f64, t22574: f64, t2314: f64, t24995: f64, t25988: f64, t26161: f64, t26558: f64, t26870: f64, t26967: f64, t27188: f64, t32235: f64, t33363: f64, t33793: f64, t33857: f64, t33928: f64, t38018: f64, t4034: f64, t4037: f64, t4072: f64, t4077: f64, t5308: f64, t652: f64, t6876: f64, t7040: f64, t7050: f64, t7056: f64, t7156: f64, t7220: f64, t7801: f64, t7890: f64, t7939: f64, t8774: f64) -> f64 {
    let t124472 = -2.0_f64 * t26967 * t2075 - 3.0_f64 * t22574 * t117084 * t25988 - 2.0_f64 * t7040 * t7890 - 2.0_f64 * t2036 * t26870 - 6.0_f64 * t24995 * t38018 * t5308 - t6876 * t33793 + t33928 * t1393 - 2.0_f64 * t32235 * t4037 + 4.0_f64 * t26161 * t26558 * t7939 * t1388 - 4.0_f64 * t2314 * t33857 - 4.0_f64 * t4034 * t33857 - 4.0_f64 * t652 * t7156 * t7801 - 2.0_f64 * t32235 * t4077 - 2.0_f64 * t652 * t8774 * t4072 + 2.0_f64 * t121210 * t2096 - 2.0_f64 * t33363 * t7220 - 4.0_f64 * t652 * t7890 * t7056 - 4.0_f64 * t102344 * t2040 - 4.0_f64 * t27188 * t7050;
    t124472
}
