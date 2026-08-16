//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 846/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk846(t243: f64, t2824: f64, t40: f64, t803: f64, t901: f64, t685: f64, t790: f64, t687: f64, t2795: f64, t286: f64, t244: f64, t2974: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11849 = t40 * t243 * t2824;
    let t11856 = t40 * t901 * t803;
    let t11869 = 1.0_f64 / t685 / t790;
    let t11870 = t687 * t687;
    let t11874 = 0.12304822629859687989e5_f64 * t286 * t11869 * t11870 * t2795;
    let t11878 = t2974 * t244;
    (t11849, t11856, t11869, t11870, t11874, t11878)
}
