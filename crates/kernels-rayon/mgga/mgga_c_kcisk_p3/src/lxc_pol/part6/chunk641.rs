//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 641/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk641(t1899: f64, t8786: f64, t1800: f64, t1869: f64, t2528: f64, t6974: f64, t2537: f64, t6719: f64, t4595: f64, t4598: f64, t7715: f64, t2364: f64, t2372: f64, t4604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8882 = t1899 * t8786;
    let t8883 = t1800 * t8882;
    let t8884 = t1869 * t8883;
    let t8886 = t6974 * t2528;
    let t8887 = t1869 * t8886;
    let t8889 = t6719 * t2537;
    let t8890 = t1869 * t8889;
    let t8896 = t4595 * t4598 * t7715;
    let t8900 = t4604 * t2364 * t2372;
    (t8882, t8883, t8884, t8886, t8887, t8889, t8890, t8896, t8900)
}
