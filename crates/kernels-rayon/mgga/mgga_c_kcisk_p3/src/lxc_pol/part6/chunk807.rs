//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 807/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk807(t4811: f64, t8886: f64, t8875: f64, t8879: f64, t8941: f64, t1692: f64, t8616: f64, t8889: f64, t5074: f64, t8867: f64, t8871: f64, t8674: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23876 = t4811 * t8886;
    let t23878 = t4811 * t8875;
    let t23880 = t4811 * t8879;
    let t23894 = t4811 * t8941;
    let t23922 = t8616 * t1692;
    let t23947 = t4811 * t8889;
    let t23949 = t5074 * t8867;
    let t23951 = t5074 * t8871;
    let t23969 = t4811 * t8674;
    (t23876, t23878, t23880, t23894, t23922, t23947, t23949, t23951, t23969)
}
