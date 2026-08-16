//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2125/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2125(t26003: f64, t4028: f64, t2314: f64, t28864: f64, t4034: f64, t1873: f64, t19289: f64, t652: f64, t1983: f64, t20085: f64, t6996: f64, t20109: f64, t20143: f64, t22461: f64, t24980: f64, t26103: f64, t28852: f64, t5460: f64, t5493: f64, t5494: f64, t6517: f64, t6862: f64, t96755: f64, t96758: f64, t96760: f64, t96763: f64, t96765: f64) -> f64 {
    let t96767 = 4.0_f64 * t4028 * t26003;
    let t96784 = 2.0_f64 * t2314 * t28864;
    let t96786 = 2.0_f64 * t4034 * t28864;
    let t96789 = 2.0_f64 * t652 * t19289 * t1873;
    let t96792 = 2.0_f64 * t1983 * t6996 * t20085;
    let t96793 = -2.0_f64 * t5493 * t652 * t6862 - 4.0_f64 * t20109 * t6517 - 2.0_f64 * t20143 * t6517 - 4.0_f64 * t22461 * t5460 - 2.0_f64 * t2314 * t28852 - 4.0_f64 * t24980 * t4028 - 4.0_f64 * t26103 * t5460 - 2.0_f64 * t26103 * t5494 - 2.0_f64 * t28852 * t4034 - t96755 - t96758 + t96760 + t96763 - t96765 - t96767 - t96784 - t96786 - t96789 + t96792;
    t96793
}
