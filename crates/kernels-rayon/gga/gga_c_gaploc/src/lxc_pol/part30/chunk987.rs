//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 987/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk987(t10721: f64, t1901: f64, t7659: f64, t9014: f64, t2508: f64, t3444: f64, t731: f64, t2958: f64, t7068: f64, t2580: f64, t1897: f64, t2549: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10722 = t1901 * t10721;
    let t10731 = t9014 * t7659;
    let t10733 = 0.92286314761706691403e-1_f64 * t2508 * t10731;
    let t10734 = t731 * t3444;
    let t10735 = 0.42725145723012357132e-3_f64 * t10734;
    let t10736 = t2958 * t7068;
    let t10737 = t2580 * t10736;
    let t10739 = 0.15381052460284448567e-1_f64 * t1897 * t10737;
    let t10740 = t2549 * t3444;
    (t10722, t10731, t10733, t10735, t10736, t10737, t10739, t10740)
}
