//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2770/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2770(t50893: f64, t162: f64, t40188: f64, t14331: f64, t40186: f64, t40203: f64, t40205: f64, t14362: f64, t9572: f64, t37: f64, t4391: f64, t2612: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t50894 = 0.10389515463408878255e3_f64 * t50893;
    let t50895 = t40188 * t162;
    let t50897 = 72.0_f64 * t50895 * t14331;
    let t50898 = 36.0_f64 * t40186;
    let t50899 = 0.35089341735807877242e1_f64 * t40203;
    let t50900 = 0.10526802520742363173e2_f64 * t40205;
    let t50901 = t14362 * t9572;
    let t50902 = 0.32530743900905219526e-1_f64 * t50901;
    let t50903 = t37 * t4391;
    let t50905 = 36.0_f64 * t50903 * t2612;
    (t50894, t50897, t50898, t50899, t50900, t50902, t50905)
}
