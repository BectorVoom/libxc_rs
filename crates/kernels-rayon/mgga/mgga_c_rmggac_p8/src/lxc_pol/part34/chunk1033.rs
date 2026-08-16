//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1033/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1033(t2447: f64, t36: f64, t321: f64, t5259: f64, t333: f64, t4669: f64, t69166: f64, t14451: f64, t1587: f64, t77883: f64, t77884: f64, t77887: f64, t77888: f64, t77889: f64, t77890: f64, t77894: f64, t77898: f64, t77899: f64, t77900: f64) -> (f64, f64) {
    let t77901 = t2447 * t36;
    let t77903 = t5259 * t77901 * t321;
    let t77904 = 0.2993560425465952141e-1_f64 * t77903;
    let t77906 = t4669 * t77901 * t333;
    let t77907 = 0.44903406381989282115e-1_f64 * t77906;
    let t77908 = 0.79828278012425390427e-1_f64 * t69166;
    let t77910 = t5259 * t14451 * t1587;
    let t77911 = 0.2993560425465952141e-1_f64 * t77910;
    let t77912 = -t77883 - t77884 + t77887 + t77888 + t77889 - 0.17961362552795712846e0_f64 * t4669 * t77890 * t321 + 0.11974241701863808564e0_f64 * t5259 * t77894 * t321 - t77898 + t77899 + t77900 - t77904 + t77907 + t77908 - t77911;
    (t77901, t77912)
}
