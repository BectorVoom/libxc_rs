//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2273/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2273(t11570: f64, t17691: f64, t15372: f64, t4889: f64, t11529: f64, t1174: f64, t6126: f64, t44571: f64, t6119: f64, t17686: f64, t44607: f64, t15382: f64, t3447: f64, t52059: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65087 = t11570 * t17691;
    let t65093 = t4889 * t15372;
    let t65112 = t1174 * t11529 * t6126;
    let t65126 = t1174 * t44571 * t6119;
    let t65128 = t44607 * t17686;
    let t65136 = t3447 * t52059 * t15382;
    (t65087, t65093, t65112, t65126, t65128, t65136)
}
