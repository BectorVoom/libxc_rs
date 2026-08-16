//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2976/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2976<F: Float>(t10482: F, t5872: F, t10413: F, t10422: F, t17924: F, t17959: F, t376: F, t10480: F, t13969: F, t17672: F, t10408: F, t1041: F, t14164: F, t14207: F, t14213: F, t14228: F, t14234: F, t17151: F, t17177: F, t17182: F, t17673: F, t17925: F, t2770: F, t3070: F, t3071: F, t3130: F, t3131: F, t42388: F, t42397: F, t42508: F, t43322: F, t4582: F, t4594: F, t4652: F, t49702: F, t62044: F, t62049: F, t62055: F, t62057: F, t62059: F, t62064: F) -> (F, F) {
    let t62079 = t5872 * t10482;
    let t62085 = t10413 * t10422 * t17924;
    let t62091 = t376 * t17959;
    let t62099 = t10480 * t13969 * t17672;
    let t62101 = t1041 * t4582 * t14164 * t62044 / F::cast_from(768.0_f64) - t62049 / F::cast_from(216.0_f64) + t14207 * t4652 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1728.0_f64) * t62055 * t62057 * t3131 * t2770 * t62059 - F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t62064 * t62057 * t14234 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t3070 * t10408 * t17177 * t14228 + F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t3070 * t42397 * t17151 * t14228 + t42508 * t17925 / F::cast_from(216.0_f64) - t49702 / F::cast_from(1728.0_f64) + t42388 * t3071 * t62079 * t14213 / F::cast_from(384.0_f64) - t62085 / F::cast_from(1728.0_f64) - t3070 * t3071 * t17182 * t14228 / F::cast_from(1152.0_f64) + t3130 * t4582 * t62091 * t4594 / F::cast_from(768.0_f64) + t43322 * t17673 / F::cast_from(256.0_f64) + t62099 / F::cast_from(384.0_f64);
    (t62091, t62101)
}
