//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1354/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1354<F: Float>(t1036: F, t10361: F, t1031: F, t10360: F, t10403: F, t1041: F, t10413: F, t10419: F, t1044: F, t10937: F, t10970: F, t248: F, t2780: F, t3041: F, t3071: F, t3077: F, t3088: F, t3132: F, t378: F, t41640: F, t41688: F, t43143: F, t43155: F, t43157: F, t43161: F, t43167: F) -> F {
    let t43176 = t10361 * t1036;
    let t43181 = -t43143 / F::cast_from(54.0_f64) + t10937 * t10419 / F::cast_from(36.0_f64) + t10403 * t3071 * t3132 * t2780 / F::cast_from(384.0_f64) - t10413 * t3071 * t3041 * t2780 / F::cast_from(768.0_f64) - F::cast_from(11.0_f64) / F::cast_from(81.0_f64) * t43155 - F::cast_from(10.0_f64) / F::cast_from(243.0_f64) * t43157 - t43161 / F::cast_from(2304.0_f64) - t1041 * t248 * t1044 * t41640 / F::cast_from(768.0_f64) + t43167 / F::cast_from(192.0_f64) - F::cast_from(5.0_f64) / F::cast_from(432.0_f64) * t1041 * t248 * t10970 * t41688 - t10360 * t1031 * t378 / F::cast_from(144.0_f64) + t43176 / F::cast_from(1152.0_f64) + F::cast_from(19.0_f64) / F::cast_from(288.0_f64) * t3077 * t3088 * t378;
    t43181
}
