//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 887/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk887<F: Float>(t28800: F, t5203: F, t1800: F, t1869: F, t6697: F, t8518: F, t1799: F, t8510: F, t5054: F, t1801: F, t28373: F, t6719: F, t8870: F) -> (F, F, F, F, F) {
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
