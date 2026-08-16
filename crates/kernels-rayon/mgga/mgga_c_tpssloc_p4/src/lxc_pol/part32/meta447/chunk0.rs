//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1708/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1708(t22751: f64, t6970: f64, t3777: f64, t6944: f64, t3787: f64, t59: f64, t240: f64, t1336: f64, t6943: f64, t835: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22752 = t22751 * t6970;
    let t22756 = t3777 * t6944;
    let t22759 = t3787 * t59;
    let t22760 = t22759 * t240;
    let t22761 = t1336 * t22760;
    let t22764 = t6943 * t835;
    let t22765 = t1336 * t22764;
    (t22752, t22756, t22759, t22760, t22761, t22764, t22765)
}
