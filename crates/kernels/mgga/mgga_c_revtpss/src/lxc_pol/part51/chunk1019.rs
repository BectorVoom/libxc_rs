//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1019/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1019<F: Float>(t11921: F, t247: F, t31913: F, t33749: F, t32010: F, t4845: F, t1042: F, t120251: F, t120361: F, t120397: F, t120403: F, t120406: F, t120448: F, t120558: F, t120625: F, t126660: F, t1669: F, t19634: F, t25698: F, t27652: F, t31891: F, t31892: F, t33751: F, t33827: F, t4757: F, t4788: F, t4976: F, t7135: F, t7145: F, t7810: F, t99970: F) -> (F,) {
    let t126689 = t31913 * t247 * t11921 * t33749;
    let t126702 = t32010 * t4845;
    let t126708 = 0.34271842599061411569e1 * t120625 * t126660 * t27652 - 0.34271842599061411569e1 * t120558 * t126660 * t4976 + 0.11423947533020470523e1 * t31891 * t31892 * t7135 * t7810 - 0.30116764542379164799e-2 * t120403 * t33751 + 0.37645955677973955999e-3 * t126689 + 0.34694512752820797848e1 * t120448 * t7145 * t99970 - 0.24791552806034007214e-3 * t120397 * t4788 - 0.52041769129231196772e1 * t25698 * t120361 * t7145 * t4757 + 0.3718732920905101082e-3 * t120406 * t33827 + 0.3718732920905101082e-3 * t126702 - 0.11156198762715303246e-2 * t120251 * t1042 * t1669 * t19634;
    (t126708,)
}
