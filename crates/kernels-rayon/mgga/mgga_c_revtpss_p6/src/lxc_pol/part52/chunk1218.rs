//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1218/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1218(t198: f64, t8665: f64, t34090: f64, t890: f64, t1962: f64, t28472: f64, t580: f64, t98631: f64, t775: f64, t102854: f64, t126007: f64, t126027: f64, t126037: f64, t1940: f64, t2403: f64, t25207: f64, t26425: f64, t27169: f64, t27173: f64, t27383: f64, t27385: f64, t28291: f64, t34080: f64, t34091: f64, t7010: f64, t7432: f64, t8657: f64, t8660: f64, t95511: f64) -> (f64, f64, f64, f64, f64) {
    let t127940 = t198 * t8665;
    let t127942 = t34090 * t890;
    let t127948 = t28472 * t98631 * t580 * t1962;
    let t127966 = t34090 * t775;
    let t127976 = t127940 * t27385 + 3.0_f64 * t26425 * t27383 * t127942 - t127948 + 3.0_f64 / 2.0_f64 * t2403 * t34080 * t7010 - t1940 * t102854 * t8660 / 2.0_f64 - t1940 * t7432 * t126027 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t95511 * t34091 + 3.0_f64 / 2.0_f64 * t2403 * t8657 * t27173 - t1940 * t7432 * t126007 / 2.0_f64 - 3.0_f64 * t28291 * t25207 * t127966 + 3.0_f64 / 2.0_f64 * t2403 * t8657 * t27169 - t1940 * t7432 * t126037 / 2.0_f64;
    (t127940, t127942, t127948, t127966, t127976)
}
