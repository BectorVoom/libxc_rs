//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1221/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1221(t15363: f64, t4889: f64, t11529: f64, t1174: f64, t6126: f64, t44571: f64, t6119: f64, t3030: f64, t6150: f64, t3609: f64, t3623: f64, t15730: f64, t5019: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65023 = t4889 * t15363;
    let t65112 = t1174 * t11529 * t6126;
    let t65126 = t1174 * t44571 * t6119;
    let t65253 = t6150 * t3030;
    let t65254 = t65253 * t3609;
    let t65262 = t65253 * t3623;
    let t65444 = t5019 * t15730;
    (t65023, t65112, t65126, t65253, t65254, t65262, t65444)
}
