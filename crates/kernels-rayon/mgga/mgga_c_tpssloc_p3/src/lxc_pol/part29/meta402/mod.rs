//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1646;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1647;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta402(t1196: f64, t12606: f64, t974: f64, t3548: f64, t4889: f64, t14736: f64, t3440: f64, t14740: f64, t11678: f64, t1174: f64, t11755: f64, t11787: f64, t11792: f64, t11794: f64, t11798: f64, t11802: f64, t11821: f64, t1227: f64, t15650: f64, t15656: f64, t15663: f64, t14731: f64, t135: f64, t5045: f64, t1222: f64, t4966: f64, t1215: f64, t1734: f64, t1089: f64, t475: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15667, t15684) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1646(t1196, t12606, t974, t3548, t4889, t14736, t3440, t14740, t11678, t1174, t11755, t11787, t11792, t11794, t11798, t11802, t11821, t1227, t15650, t15656, t15663);
        let (t15686, t15689, t15691, t15699, t15700, t15702) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1647(t14731, t3440, t135, t5045, t1174, t1222, t4966, t1215, t1734, t1089, t475, t607);
    (t15667, t15684, t15686, t15689, t15691, t15699, t15700, t15702)
}
