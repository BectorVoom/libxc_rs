//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 887/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk887(t28800: f64, t5203: f64, t1800: f64, t1869: f64, t6697: f64, t8518: f64, t1799: f64, t8510: f64, t5054: f64, t1801: f64, t28373: f64, t6719: f64, t8870: f64) -> (f64, f64, f64, f64, f64) {
    let t28801 = t5203 * t28800;
    let t28802 = t1800 * t28801;
    let t28803 = t1869 * t28802;
    let t28805 = t6697 * t8518;
    let t28806 = t1800 * t28805;
    let t28807 = t1799 * t28806;
    let t28809 = t6697 * t8510;
    let t28810 = t1800 * t28809;
    let t28811 = t5054 * t28810;
    let t28813 = t1801 * t28373;
    let t28814 = t1800 * t28813;
    let t28815 = t5054 * t28814;
    let t28817 = t6719 * t8870;
    (t28803, t28807, t28811, t28815, t28817)
}
