//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1143/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1143(t762: f64, t88192: f64, t88215: f64, t89044: f64, t89089: f64, t5147: f64, t2568: f64, t10007: f64, t10157: f64, t1091: f64, t1175: f64, t14200: f64, t1901: f64, t21416: f64, t21486: f64, t242: f64, t265: f64, t446: f64, t729: f64, t80345: f64, t80399: f64, t80406: f64, t80412: f64, t80429: f64, t88196: f64, t88939: f64) -> (f64, f64, f64) {
    let t89092 = t762 * (t88192 + t88215 + t89044 + t89089);
    let t89096 = t5147 * t5147;
    let t89097 = t2568 * t89096;
    let t89117 = -8.0_f64 * t446 * t10157 * t1175 * t21416 - t446 * t242 * t89092 / 3.0_f64 + 2.0_f64 * t446 * t242 * t89097 + 8.0_f64 / 9.0_f64 * t80345 - t446 * t729 * t265 * t88939 / 3.0_f64 - 4.0_f64 / 3.0_f64 * t80399 - 4.0_f64 / 9.0_f64 * t80406 + 8.0_f64 / 9.0_f64 * t1901 * t14200 * t88196 - 4.0_f64 / 3.0_f64 * t1901 * t10007 * t21486 * t1091 + 8.0_f64 / 9.0_f64 * t80412 - 8.0_f64 / 27.0_f64 * t80429;
    (t89092, t89097, t89117)
}
