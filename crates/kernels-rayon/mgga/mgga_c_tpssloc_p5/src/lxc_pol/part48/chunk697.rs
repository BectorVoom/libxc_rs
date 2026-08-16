//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 697/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk697(t22751: f64, t6970: f64, t3853: f64, t6945: f64, t3777: f64, t6944: f64, t1354: f64, t3787: f64, t59: f64, t240: f64, t1336: f64, t3795: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22752 = t22751 * t6970;
    let t22753 = 0.76763589786250567036e-1_f64 * t22752;
    let t22754 = t6945 * t3853;
    let t22756 = t3777 * t6944;
    let t22757 = t22756 * t1354;
    let t22759 = t3787 * t59;
    let t22760 = t22759 * t240;
    let t22761 = t1336 * t22760;
    let t22762 = t22761 * t3795;
    (t22752, t22753, t22754, t22757, t22759, t22762)
}
