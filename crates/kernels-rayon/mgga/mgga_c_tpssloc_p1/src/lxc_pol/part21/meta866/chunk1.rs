//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3161/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3161(t3590: f64, t6224: f64, t11877: f64, t11904: f64, t11907: f64, t1244: f64, t1246: f64, t14989: f64, t15004: f64, t15027: f64, t15032: f64, t15248: f64, t19123: f64, t19139: f64, t19189: f64, t19201: f64, t19204: f64, t3617: f64, t3624: f64, t3625: f64, t5011: f64, t5052: f64, t5064: f64, t5079: f64, t5080: f64, t5084: f64, t52435: f64, t53565: f64, t6261: f64) -> (f64, f64) {
    let t65347 = t3590 * t6224;
    let t65374 = 4.0_f64 * t1244 * t1246 * t5011 * t5052 - 2.0_f64 * t19189 * t3624 * t5079 - t3624 * t3625 * t65347 + t11877 * t6261 + 4.0_f64 * t11904 * t19123 + 8.0_f64 * t11904 * t19204 - 4.0_f64 * t11907 * t19139 + 4.0_f64 * t14989 * t5064 + 8.0_f64 * t15004 * t15027 + 4.0_f64 * t15032 * t5084 - 12.0_f64 * t15248 * t53565 + 2.0_f64 * t19201 * t3617 - 4.0_f64 * t5080 * t52435;
    (t65347, t65374)
}
