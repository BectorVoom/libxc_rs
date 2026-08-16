//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1995;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta447(t1089: f64, t1215: f64, t607: f64, t15659: f64, t3578: f64, t1196: f64, t12606: f64, t974: f64, t3548: f64, t4889: f64, t14736: f64, t3440: f64, t14740: f64, t11678: f64, t1174: f64, t11755: f64, t11787: f64, t11792: f64, t11794: f64, t11798: f64, t11802: f64, t11821: f64, t1227: f64, t15650: f64, t15656: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15661, t15662, t15663, t15666, t15667, t15671, t15672) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1995(t1089, t1215, t607, t15659, t3578, t1196, t12606, t974, t3548, t4889, t14736, t3440);
        let (t15681, t15684) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1996(t14740, t3440, t11678, t1174, t11755, t11787, t11792, t11794, t11798, t11802, t11821, t1227, t15650, t15656, t15663, t15667, t15671, t15672);
    (t15661, t15662, t15663, t15666, t15667, t15671, t15672, t15681, t15684)
}
