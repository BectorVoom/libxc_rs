//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 996/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk996(t10932: f64, t179: f64, t5634: f64, t2889: f64, t3515: f64, t2888: f64, t1123: f64, t3638: f64, t301: f64, t5934: f64, t758: f64, t10833: f64, t1976: f64, t722: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10934 = t179 * t5634 * t10932;
    let t10937 = t2889 * t3515;
    let t10938 = t2888 * t10937;
    let t10942 = t3638 * t1123;
    let t10943 = t301 * t10942;
    let t10944 = t10943 * t5934;
    let t10945 = t758 * t10944;
    let t10949 = t1976 * t10833 * t722;
    (t10934, t10937, t10938, t10942, t10943, t10944, t10945, t10949)
}
