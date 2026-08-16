//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 828/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk828(t1417: f64, t7874: f64, t7899: f64, t7866: f64, t3739: f64, t7908: f64, t3748: f64, t8181: f64, t7839: f64, t7833: f64, t13959: f64, t8172: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26712 = t1417 * t7874;
    let t26714 = t1417 * t7899;
    let t26746 = t1417 * t7866;
    let t26755 = t3739 * t7908;
    let t26764 = t3748 * t8181;
    let t26785 = t3739 * t7839;
    let t26787 = t3739 * t7833;
    let t26841 = t13959 * t8172;
    (t26712, t26714, t26746, t26755, t26764, t26785, t26787, t26841)
}
