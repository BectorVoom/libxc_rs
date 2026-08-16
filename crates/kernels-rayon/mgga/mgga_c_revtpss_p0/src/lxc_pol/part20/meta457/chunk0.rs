//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1742/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1742(t1331: f64, t9342: f64, t9855: f64, t2619: f64, t9563: f64, t3825: f64, t9586: f64, t1333: f64, t14: f64, t27: f64, t521: f64, t583: f64, t596: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47005 = t9342 * t1331;
    let t47006 = 96.0_f64 * t47005;
    let t47007 = t9855 * t1331;
    let t47008 = 576.0_f64 * t47007;
    let t47009 = t9563 * t2619;
    let t47010 = 0.14649157844805236043e-2_f64 * t47009;
    let t47011 = t3825 * t9586;
    let t47012 = 0.22787578869697033845e-2_f64 * t47011;
    let t47013 = t9342 * t1333;
    let t47014 = 96.0_f64 * t47013;
    let t47016 = t14 * t27 * t521;
    let t47017 = 1440.0_f64 * t47016;
    let t47019 = t583 * t596 * t521;
    (t47006, t47008, t47010, t47012, t47014, t47017, t47019)
}
