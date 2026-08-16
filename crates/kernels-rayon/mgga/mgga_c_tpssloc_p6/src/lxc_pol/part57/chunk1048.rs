//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1048/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1048(t120857: f64, t122852: f64, t122853: f64, t122856: f64, t122857: f64, t122860: f64, t122862: f64, t122864: f64, t127695: f64, t128976: f64, t128989: f64, t1398: f64, t1852: f64, t1858: f64, t2023: f64, t2029: f64, t2099: f64, t2105: f64, t28869: f64, t28904: f64, t29396: f64, t29430: f64, t3: f64, t33628: f64, t33662: f64, t580: f64, t6471: f64, t6483: f64, t7759: f64, t7774: f64, t7946: f64, t7961: f64, t8647: f64, t8660: f64) -> f64 {
    let tv4rho2sigma213 = 2.0_f64 * t122864 + t29396 * t2029 + 2.0_f64 * t33628 * t1858 + t28869 * t2105 + t2023 * t29430 + t8647 * t6483 + 2.0_f64 * t122853 + 2.0_f64 * t7946 * t7774 + 2.0_f64 * t122852 + t6471 * t8660 + 2.0_f64 * t122860 + 2.0_f64 * t7759 * t7961 + t1398 * (t127695 + t128989) + t2099 * t28904 + t3 * t128976 * t580 + 2.0_f64 * t120857 + 2.0_f64 * t1852 * t33662 + 2.0_f64 * t122857 + 2.0_f64 * t122862 + 2.0_f64 * t122856;
    tv4rho2sigma213
}
