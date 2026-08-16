//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1995/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1995(t1089: f64, t1215: f64, t607: f64, t15659: f64, t3578: f64, t1196: f64, t12606: f64, t974: f64, t3548: f64, t4889: f64, t14736: f64, t3440: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15660 = t1215 * t1089;
    let t15661 = t15660 * t607;
    let t15662 = t15659 * t15661;
    let t15663 = t3578 * t15662;
    let t15666 = t1196 * t12606;
    let t15667 = t974 * t15666;
    let t15671 = t4889 * t3548 / 162.0_f64;
    let t15672 = t3440 * t14736;
    (t15661, t15662, t15663, t15666, t15667, t15671, t15672)
}
