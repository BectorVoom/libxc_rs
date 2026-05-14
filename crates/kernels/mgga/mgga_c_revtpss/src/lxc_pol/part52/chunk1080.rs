//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1080/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1080<F: Float>(t198: F, t8665: F, t34090: F, t890: F, t1962: F, t28472: F, t580: F, t98631: F, t775: F, t102854: F, t126007: F, t126027: F, t126037: F, t1940: F, t2403: F, t25207: F, t26425: F, t27169: F, t27173: F, t27383: F, t27385: F, t28291: F, t34080: F, t34091: F, t7010: F, t7432: F, t8657: F, t8660: F, t95511: F) -> (F, F, F, F, F) {
    let t127940 = t198 * t8665;
    let t127942 = t34090 * t890;
    let t127948 = t28472 * t98631 * t580 * t1962;
    let t127966 = t34090 * t775;
    let t127976 = t127940 * t27385 + 3.0 * t26425 * t27383 * t127942 - t127948 + 3.0 / 2.0 * t2403 * t34080 * t7010 - t1940 * t102854 * t8660 / 2.0 - t1940 * t7432 * t126027 / 2.0 - 3.0 / 2.0 * t95511 * t34091 + 3.0 / 2.0 * t2403 * t8657 * t27173 - t1940 * t7432 * t126007 / 2.0 - 3.0 * t28291 * t25207 * t127966 + 3.0 / 2.0 * t2403 * t8657 * t27169 - t1940 * t7432 * t126037 / 2.0;
    (t127940, t127942, t127948, t127966, t127976)
}
