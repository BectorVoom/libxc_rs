//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 838/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk838(t27733: f64, t7452: f64, t1127: f64, t7590: f64, t1109: f64, t12: f64, t52: f64, t237: f64, t6762: f64, t1097: f64, t1111: f64, t1113: f64, t1408: f64, t1412: f64, t1420: f64, t17987: f64, t2035: f64, t238: f64, t30816: f64, t33351: f64, t6763: f64, t6774: f64, t6778: f64, t6795: f64, t7448: f64, t7456: f64, t7458: f64) -> (f64, f64, f64, f64) {
    let t35481 = t27733 * t7452;
    let t35490 = t7590 * t1127;
    let t35504 = t12 * t1109;
    let t35505 = t52 * t35504;
    let t35508 = t6762 * t237;
    let t35513 = -0.76612330055555555556e-1_f64 * t35481 * t1420 + 0.11854761295685025975e-1_f64 * t7448 * t1111 + 0.39525571512470170088e-4_f64 * t30816 * t2035 * t7590 * t1113 - 0.19762785756235085044e-4_f64 * t17987 * t2035 * t35490 - 0.88910709717637694816e-2_f64 * t6763 * t1408 - 0.88910709717637694816e-2_f64 * t6778 * t1408 - 0.21080304806650757379e-3_f64 * t1412 * t6795 + 0.47419045182740103902e-1_f64 * t1412 * t6774 - 0.23254900946437792e-1_f64 * t33351 * t1097 - 0.11854761295685025975e-1_f64 * t7456 * t35505 + 0.22227677429409423704e-2_f64 * t35508 * t7458 + 0.11854761295685025975e-1_f64 * t238 * t35505;
    (t35481, t35504, t35505, t35513)
}
