//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta355 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1270;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1271;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta355(t11153: f64, t3584: f64, t1734: f64, t3508: f64, t3548: f64, t4889: f64, t135: f64, t5045: f64, t1174: f64, t1222: f64, t4966: f64, t1215: f64, t1089: f64, t475: f64, t1744: f64, t3540: f64, t1731: f64, t4961: f64, t1706: f64, t3545: f64, t11818: f64, t1735: f64, t248: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15654, t15659, t15671, t15691, t15699, t15700) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1270(t11153, t3584, t1734, t3508, t3548, t4889, t135, t5045, t1174, t1222, t4966, t1215);
        let (t15701, t15717, t15719, t15722, t15727, t15730) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1271(t1089, t475, t1744, t3540, t1731, t1222, t4961, t1706, t3545, t11818, t1735, t248);
    (t15654, t15659, t15671, t15691, t15699, t15700, t15701, t15717, t15719, t15722, t15727, t15730)
}
