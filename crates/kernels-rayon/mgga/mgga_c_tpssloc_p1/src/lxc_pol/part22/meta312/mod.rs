//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta312 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1488;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1489;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta312(t15281: f64, t4936: f64, t1174: f64, t3431: f64, t4912: f64, t11583: f64, t3961: f64, t11529: f64, t1709: f64, t3432: f64, t4889: f64, t3450: f64, t3966: f64, t3448: f64, t4928: f64, t11588: f64, t1714: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15282, t15284, t15285, t15287, t15293, t15299, t15300, t15307, t15313) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1488(t15281, t4936, t1174, t3431, t4912, t11583, t3961, t11529, t1709, t3432, t4889, t3450, t3966);
        let (t15320, t15338) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1489(t3448, t4928, t11588, t1714);
    (t15282, t15284, t15285, t15287, t15293, t15299, t15300, t15307, t15313, t15320, t15338)
}
