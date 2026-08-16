//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2634/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2634(t11913: f64, t52834: f64, t11880: f64, t11712: f64, t11887: f64, t491: f64, t11638: f64, t11871: f64, t11877: f64, t11897: f64, t11904: f64, t1244: f64, t1246: f64, t14997: f64, t15022: f64, t15032: f64, t15430: f64, t15777: f64, t1755: f64, t1932: f64, t3493: f64, t3604: f64, t3610: f64, t3621: f64, t3624: f64, t45329: f64, t475: f64, t5052: f64, t5064: f64, t5083: f64, t5084: f64, t52480: f64, t52709: f64) -> (f64, f64, f64) {
    let t53592 = t52834 * t11913;
    let t53613 = t52834 * t11880;
    let t53646 = t11712 * t11887 * t491;
    let t53650 = -t11638 * t1755 * t1932 * t3624 * t475 + 3.0_f64 * t1244 * t1246 * t3493 * t5052 + 6.0_f64 * t11871 * t3610 * t5083 - 3.0_f64 * t15022 * t3624 * t5083 - 18.0_f64 * t52480 * t52709 * t53646 + 3.0_f64 * t11877 * t5084 + 3.0_f64 * t11897 * t5064 + 12.0_f64 * t11904 * t14997 + 3.0_f64 * t15032 * t3621 + 3.0_f64 * t15430 * t45329 + 6.0_f64 * t15777 * t3604;
    (t53592, t53613, t53650)
}
