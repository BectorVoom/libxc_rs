//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1233/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1233(t7787: f64, t92794: f64, t1094: f64, t283: f64, t1130: f64, t15573: f64, t27089: f64, t7788: f64, t27055: f64, t27070: f64, t26707: f64, t2822: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92795 = t7787 * t92794;
    let t92807 = t1094 * t283;
    let t92808 = t92807 * t1130;
    let t92814 = t7788 * t15573 * t27089;
    let t92816 = t27070 * t27055;
    let t92818 = t2822 * t26707;
    (t92795, t92807, t92808, t92814, t92816, t92818)
}
