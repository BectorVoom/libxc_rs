//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1167/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1167(t31552: f64, t30204: f64, t6525: f64, t7967: f64, t3338: f64, t447: f64, t2366: f64, t10119: f64, t10127: f64, t1349: f64, t2268: f64, t2343: f64, t31522: f64, t31525: f64, t31527: f64, t31533: f64, t31534: f64, t31539: f64, t31542: f64, t31546: f64, t31551: f64, t3818: f64, t4323: f64, t6313: f64) -> (f64, f64, f64) {
    let t31553 = 0.94850022118920498665e-2_f64 * t31552;
    let t31555 = t6525 * t30204 * t7967;
    let t31556 = 0.47425011059460249332e-2_f64 * t31555;
    let t31557 = t3338 * t447;
    let t31558 = t2366 * t31557;
    let t31562 = t31522 + t31525 + t31527 - 0.7588001769513639893e-1_f64 * t3818 * t10119 + 0.7588001769513639893e-1_f64 * t6313 * t10127 + t31533 + 0.56910013271352299198e-1_f64 * t2268 * t2343 * t31534 + t31539 + t31542 + t31546 - t31551 + t31553 - t31556 - 0.63233348079280332442e-2_f64 * t1349 * t4323 * t31558;
    (t31557, t31558, t31562)
}
