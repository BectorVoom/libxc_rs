//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 833/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk833(t35409: f64, t420: f64, t1127: f64, t230: f64, t7470: f64, t27729: f64, t6: f64, t3766: f64, t33444: f64, t1113: f64, t683: f64, t224: f64, t2427: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35410 = t420 * t35409;
    let t35414 = t230 * t1127;
    let t35415 = t420 * t35414;
    let t35416 = t7470 * t35415;
    let t35419 = t27729 * t6;
    let t35420 = t3766 * t35419;
    let t35426 = t3766 * t33444;
    let t35427 = t683 * t1113;
    let t35431 = t683 * t1127;
    let t35435 = t224 * t2427;
    (t35410, t35414, t35415, t35416, t35420, t35426, t35427, t35431, t35435)
}
