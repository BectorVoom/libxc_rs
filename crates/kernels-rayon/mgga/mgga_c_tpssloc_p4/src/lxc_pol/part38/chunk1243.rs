//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1243/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1243(t11888: f64, t11904: f64, t11907: f64, t11914: f64, t1201: f64, t1244: f64, t1247: f64, t15032: f64, t15241: f64, t15245: f64, t15248: f64, t15253: f64, t15257: f64, t15426: f64, t15430: f64, t15772: f64, t15777: f64, t1758: f64, t3565: f64, t3604: f64, t3610: f64, t3621: f64, t3624: f64, t3626: f64, t470: f64, t494: f64, t5064: f64, t5069: f64, t5076: f64, t5080: f64, t5084: f64, t5086: f64) -> f64 {
    let t15785 = 2.0_f64 * t15032 * t1247 + t1244 * t15241 - 2.0_f64 * t11907 * t5080 - t15245 * t3626 - 6.0_f64 * t11888 * t15248 + 2.0_f64 * t3604 * t5076 + 2.0_f64 * t3610 * t15253 + t3565 * t1758 - 2.0_f64 * t3624 * t15257 + t15426 * t494 + t11914 * t15430 + t470 * t15772 + 2.0_f64 * t1201 * t5086 + 2.0_f64 * t1244 * t15777 + t5064 * t3621 + 2.0_f64 * t3604 * t5084 + 4.0_f64 * t11904 * t5069;
    t15785
}
