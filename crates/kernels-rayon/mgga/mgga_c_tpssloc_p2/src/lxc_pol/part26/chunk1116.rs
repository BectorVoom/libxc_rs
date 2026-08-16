//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1116/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1116(t1906: f64, t23012: f64, t2679: f64, t6657: f64, t1894: f64, t2710: f64, t214: f64, t1880: f64, t1909: f64, t22984: f64, t22990: f64, t22993: f64, t23000: f64, t23003: f64, t23006: f64, t23009: f64, t2613: f64, t2617: f64, t6658: f64, t6660: f64, t808: f64, t812: f64) -> (f64, f64, f64, f64) {
    let t23013 = t23012 * t1906;
    let t23014 = 0.63969658155208805863e-1_f64 * t23013;
    let t23016 = t6657 * t2679;
    let t23020 = t1894 * t2710;
    let t23021 = t214 * t23020;
    let t23022 = t1880 * t23021;
    let t23024 = 2.0_f64 * t808 * t6660 - t812 * t22984 + 0.3289868133696452873e-1_f64 * t22990 - 2.0_f64 * t812 * t22993 + 0.16449340668482264365e-1_f64 * t23000 + t23003 - 0.82246703342411321825e-2_f64 * t23006 + 2.0_f64 * t812 * t23009 + t23014 + t2613 * t1909 - t812 * t23016 - 2.0_f64 * t2617 * t6658 + 0.82246703342411321825e-2_f64 * t23022;
    (t23016, t23020, t23021, t23024)
}
