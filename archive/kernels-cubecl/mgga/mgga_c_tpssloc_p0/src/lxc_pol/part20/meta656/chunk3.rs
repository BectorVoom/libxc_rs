//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2427/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2427<F: Float>(t1615: F, t3120: F, t3040: F, t10403: F, t10422: F, t14214: F, t3030: F, t4552: F, t3032: F, t3129: F, t1022: F, t10408: F, t10413: F, t10937: F, t14174: F, t14207: F, t14211: F, t14212: F, t14220: F, t14222: F, t14235: F, t14491: F, t2244: F, t2250: F, t2770: F, t3071: F, t3114: F, t3117: F, t3123: F, t3134: F, t42483: F, t42508: F, t42530: F, t4337: F, t49594: F) -> (F, F, F, F) {
    let t49616 = t1615 * t3120;
    let t49621 = t1615 * t3040;
    let t49629 = t10403 * t10422 * t14214;
    let t49649 = t4552 * t3030;
    let t49650 = t49649 * t3032;
    let t49651 = t49650 * t3129;
    let t49654 = F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t10403 * t10408 * t14211 * t1022 * t2770 * t2244 - t10413 * t3071 * t49616 * t14220 / F::cast_from(1536.0_f64) + t42483 * t3071 * t49621 * t14220 / F::cast_from(1536.0_f64) - F::cast_from(5.0_f64) / F::cast_from(432.0_f64) * t10937 * t14235 + t49629 / F::cast_from(576.0_f64) + t42508 * t14222 / F::cast_from(144.0_f64) + t10403 * t3071 * t14211 * t14212 * t2250 / F::cast_from(768.0_f64) + t42530 / F::cast_from(864.0_f64) - F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t10413 * t10408 * t4337 * t49594 + t3114 * t14491 / F::cast_from(1024.0_f64) - F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t3117 * t14174 + t14207 * t3123 / F::cast_from(1024.0_f64) + t49651 * t3134 / F::cast_from(512.0_f64);
    (t49616, t49649, t49650, t49654)
}
