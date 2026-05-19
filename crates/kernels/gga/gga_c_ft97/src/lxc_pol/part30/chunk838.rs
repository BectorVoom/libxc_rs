//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 838/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk838<F: Float>(t27733: F, t7452: F, t1127: F, t7590: F, t1109: F, t12: F, t52: F, t237: F, t6762: F, t1097: F, t1111: F, t1113: F, t1408: F, t1412: F, t1420: F, t17987: F, t2035: F, t238: F, t30816: F, t33351: F, t6763: F, t6774: F, t6778: F, t6795: F, t7448: F, t7456: F, t7458: F) -> (F, F, F, F) {
    let t35481 = t27733 * t7452;
    let t35490 = t7590 * t1127;
    let t35504 = t12 * t1109;
    let t35505 = t52 * t35504;
    let t35508 = t6762 * t237;
    let t35513 = -F::cast_from(0.76612330055555555556e-1_f64) * t35481 * t1420 + F::cast_from(0.11854761295685025975e-1_f64) * t7448 * t1111 + F::cast_from(0.39525571512470170088e-4_f64) * t30816 * t2035 * t7590 * t1113 - F::cast_from(0.19762785756235085044e-4_f64) * t17987 * t2035 * t35490 - F::cast_from(0.88910709717637694816e-2_f64) * t6763 * t1408 - F::cast_from(0.88910709717637694816e-2_f64) * t6778 * t1408 - F::cast_from(0.21080304806650757379e-3_f64) * t1412 * t6795 + F::cast_from(0.47419045182740103902e-1_f64) * t1412 * t6774 - F::cast_from(0.23254900946437792e-1_f64) * t33351 * t1097 - F::cast_from(0.11854761295685025975e-1_f64) * t7456 * t35505 + F::cast_from(0.22227677429409423704e-2_f64) * t35508 * t7458 + F::cast_from(0.11854761295685025975e-1_f64) * t238 * t35505;
    (t35481, t35504, t35505, t35513)
}
