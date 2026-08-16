//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1143/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1143(t19803: f64, t1009: f64, t5137: f64, t16638: f64, t1634: f64, t637: f64, t1508: f64, t7035: f64, t496: f64, t6825: f64, t2562: f64, t500: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19804 = 240.0_f64 * t19803;
    let t19805 = t5137 * t1009;
    let t19806 = 120.0_f64 * t19805;
    let t19807 = 180.0_f64 * t16638;
    let t19809 = t1634 * t637;
    let t19822 = t7035 * t1508;
    let t19823 = 0.51947577317044391276e2_f64 * t19822;
    let t19824 = t496 * t6825;
    let t19825 = 12.0_f64 * t19824;
    let t19843 = 16.0_f64 * t2562 * t500;
    (t19804, t19806, t19807, t19809, t19823, t19825, t19843)
}
