//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2579/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2579(t11583: f64, t12652: f64, t12648: f64, t11570: f64, t14165: f64, t44607: f64, t10913: f64, t4723: f64, t11536: f64, t4889: f64, t1174: f64, t15268: f64, t15281: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t52216 = t11583 * t12652;
    let t52220 = t11583 * t12648;
    let t52224 = t11570 * t14165;
    let t52228 = t44607 * t14165;
    let t52236 = t4723 * t10913;
    let t52240 = t4889 * t11536;
    let t52250 = t1174 * t15281 * t15268;
    (t52216, t52220, t52224, t52228, t52236, t52240, t52250)
}
