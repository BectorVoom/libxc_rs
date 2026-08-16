//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1236/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1236(t178: f64, t18152: f64, t5953: f64, t5719: f64, t2899: f64, t7728: f64, t774: f64, t7732: f64, t7736: f64, t7738: f64, t7742: f64, t7744: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21603 = t18152 * t178;
    let t21604 = t5953 * t21603;
    let t21607 = t5719 * t21603;
    let t21611 = t2899 * t774 * t7728;
    let t21614 = t2899 * t774 * t7732;
    let t21617 = t7736 * t774 * t7738;
    let t21620 = t7742 * t774 * t7744;
    (t21603, t21604, t21607, t21611, t21614, t21617, t21620)
}
