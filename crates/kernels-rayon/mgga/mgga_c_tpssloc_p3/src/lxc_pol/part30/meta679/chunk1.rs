//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2128/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2128(t26161: f64, t26162: f64, t96830: f64, t26114: f64, t7468: f64, t26179: f64, t1266: f64, t1980: f64, t20098: f64, t27996: f64, t28811: f64, t510: f64, t5450: f64, t650: f64, t652: f64, t671: f64, t6862: f64, t96655: f64, t96796: f64, t96799: f64, t96802: f64, t96805: f64, t96807: f64, t96813: f64, t96815: f64, t96818: f64, t96827: f64, t96829: f64) -> f64 {
    let t96833 = 4.0_f64 * t26161 * t26162 * t96830;
    let t96837 = 4.0_f64 * t26114 * t7468;
    let t96839 = 4.0_f64 * t26179 * t7468;
    let t96840 = -2.0_f64 * t28811 * t652 * t671 - 2.0_f64 * t1266 * t27996 + t1980 * t20098 - t28811 * t650 - 2.0_f64 * t510 * t96655 - t5450 * t6862 + t96796 + t96799 - t96802 + t96805 - t96807 - t96813 - t96815 - t96818 + t96827 - t96829 + t96833 - t96837 - t96839;
    t96840
}
