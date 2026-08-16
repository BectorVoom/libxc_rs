//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2446/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2446<F: Float>(t3082: F, t4617: F, t3132: F, t607: F, t3120: F, t4594: F, t10904: F, t14025: F, t10403: F, t10408: F, t1041: F, t10937: F, t13975: F, t13980: F, t13991: F, t14009: F, t14230: F, t1539: F, t2960: F, t3070: F, t3071: F, t3130: F, t42334: F, t42522: F, t43241: F, t4337: F, t4342: F, t4582: F, t4583: F, t4596: F, t45997: F, t48506: F) -> (F, F) {
    let t49993 = t4617 * t3082;
    let t49994 = t49993 / F::cast_from(4608.0_f64);
    let t50009 = t3132 * t607;
    let t50014 = t4594 * t3120;
    let t50027 = t10904 * t14025;
    let t50035 = -t49994 - F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t42334 * t13991 + t3130 * t4582 * t48506 * t4594 / F::cast_from(512.0_f64) + t3130 * t4582 * t13975 * t13980 / F::cast_from(512.0_f64) - t3070 * t3071 * t4342 * t43241 / F::cast_from(768.0_f64) - t10403 * t3071 * t4342 * t50009 / F::cast_from(384.0_f64) + t10403 * t3071 * t1539 * t50014 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t10403 * t10408 * t4337 * t50009 + t10937 * t14230 / F::cast_from(72.0_f64) + F::cast_from(19.0_f64) / F::cast_from(288.0_f64) * t42522 * t4596 - t50027 / F::cast_from(72.0_f64) - t1041 * t4582 * t4583 * t45997 / F::cast_from(768.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2960 * t14009;
    (t50014, t50035)
}
