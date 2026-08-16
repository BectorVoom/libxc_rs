//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1411;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1412;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta361(t3403: f64, t4857: f64, t15026: f64, t3623: f64, t1706: f64, t3428: f64, t135: f64, t457: f64, t4936: f64, t1174: f64, t3431: f64, t4912: f64, t11583: f64, t3961: f64, t11529: f64, t1709: f64, t3432: f64, t4889: f64, t3450: f64, t3966: f64, t3448: f64, t4928: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15218, t15245, t15265, t15284, t15285) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1411(t3403, t4857, t15026, t3623, t1706, t3428, t135, t457, t4936, t1174, t3431, t4912);
        let (t15287, t15293, t15300, t15307, t15313, t15320) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1412(t1174, t15285, t11583, t3961, t11529, t1709, t3432, t4889, t3450, t3966, t3448, t4928);
    (t15218, t15245, t15265, t15284, t15287, t15293, t15300, t15307, t15313, t15320)
}
