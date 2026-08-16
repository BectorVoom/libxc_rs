//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2258/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2258(t23788: f64, t86797: f64, t16596: f64, t83555: f64, t1081: f64, t4303: f64, t28: f64, t40772: f64, t86717: f64, t1877: f64, t22959: f64, t23781: f64, t23807: f64, t23810: f64, t23813: f64, t25013: f64, t2522: f64, t25354: f64, t25358: f64, t25372: f64, t25892: f64, t25898: f64, t25905: f64, t4314: f64, t6666: f64, t6670: f64, t6841: f64, t7541: f64, t81483: f64, t86740: f64, t86775: f64, t86835: f64, t87975: f64) -> f64 {
    let t89928 = t23788 * t86797;
    let t89931 = t83555 * t16596;
    let t89941 = t1081 * t4303;
    let t89953 = t40772 * t28;
    let t89954 = t89953 * t86717;
    let t89957 = 3.0_f64 * t4314 * t7541 * t23781 - t1877 * t25358 * t23813 / 2.0_f64 - t86775 - 6.0_f64 * t25013 * t89928 - 3.0_f64 * t22959 * t89931 + 3.0_f64 * t2522 * t6666 * t25905 + t1877 * t87975 * t23807 + t1877 * t25354 * t1081 - t1877 * t6670 * t89941 - t1877 * t25358 * t23810 + 3.0_f64 * t2522 * t25354 * t6841 - 3.0_f64 * t81483 * t25898 - t86835 + 6.0_f64 * t86740 * t25892 - 3.0_f64 * t25372 * t89954;
    t89957
}
