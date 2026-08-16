//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 429/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk429(t2268: f64, t787: f64, t269: f64, t348: f64, t2059: f64, t737: f64, t2078: f64, t257: f64, t751: f64, t105: f64, t107: f64, t1308: f64, t1312: f64, t1319: f64, t2141: f64, t260: f64, t438: f64, t446: f64, t447: f64, t780: f64) -> (f64, f64) {
    let t2269 = t2268 * t787;
    let t2281 = t348 * t269;
    let t2287 = t737 * t2059;
    let t2291 = t257 * t2078;
    let t2295 = t751 * t751;
    let t2299 = -0.11281315546296296296e-3_f64 * t105 * t1308 * t269 + 0.1e-22_f64 * t446 * t1312 * t269 - 0.67687893277777777778e-3_f64 * t105 * t438 * t780 + 0.50765919958333333334e-3_f64 * t1319 * t2281 + 0.50765919958333333334e-3_f64 * t446 * t447 * t780 + 0.10153183991666666667e-2_f64 * t105 * t107 * t2287 - 0.50765919958333333334e-3_f64 * t105 * t107 * t2291 - 4.0_f64 * t2295 - 4.0_f64 * t260 * t2141;
    (t2269, t2299)
}
