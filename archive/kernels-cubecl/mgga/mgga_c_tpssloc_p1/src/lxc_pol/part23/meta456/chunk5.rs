//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1324/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1324<F: Float>(t76359: F, t76371: F, t225: F, t13222: F, t13228: F, t1512: F, t20953: F, t237: F, t249: F, t4167: F, t4178: F, t59259: F, t59263: F, t59276: F, t59288: F, t67872: F, t68148: F, t68195: F, t68197: F, t68199: F, t68201: F, t76250: F) -> (F, F, F) {
    let t76372 = t76359 + t76371;
    let t76373 = t76372 * t225;
    let t76394 = t76373 * t237 * t249 / F::cast_from(3072.0_f64) - F::cast_from(7.0_f64) / F::cast_from(4.0_f64) * t68148 - F::cast_from(119.0_f64) / F::cast_from(288.0_f64) * t59259 - F::cast_from(119.0_f64) / F::cast_from(576.0_f64) * t59263 - t4167 * t20953 / F::cast_from(768.0_f64) - t67872 * t1512 / F::cast_from(768.0_f64) - F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t59276 + F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t59288 + F::cast_from(35.0_f64) / F::cast_from(48.0_f64) * t68195 - F::cast_from(35.0_f64) / F::cast_from(96.0_f64) * t68197 + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t68199 + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t68201 - t4178 * t13222 * t13228 * t76250 / F::cast_from(32.0_f64);
    (t76372, t76373, t76394)
}
