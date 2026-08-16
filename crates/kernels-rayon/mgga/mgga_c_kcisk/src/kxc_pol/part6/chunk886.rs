//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 886/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk886(t1801: f64, t28381: f64, t1800: f64, t1799: f64, t6697: f64, t8514: f64, t15858: f64, t8480: f64, t5182: f64, t2441: f64, t8786: f64) -> (f64, f64, f64, f64) {
    let t28788 = t1801 * t28381;
    let t28789 = t1800 * t28788;
    let t28790 = t1799 * t28789;
    let t28792 = t6697 * t8514;
    let t28793 = t1800 * t28792;
    let t28794 = t1799 * t28793;
    let t28796 = t15858 * t8480;
    let t28797 = t5182 * t28796;
    let t28800 = t2441 * t8786;
    (t28790, t28794, t28797, t28800)
}
