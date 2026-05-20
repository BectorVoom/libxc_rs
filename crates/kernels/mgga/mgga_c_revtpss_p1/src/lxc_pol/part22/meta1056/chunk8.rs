//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3745/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3745<F: Float>(t1715: F, t3601: F, t20816: F, t3708: F, t1121: F, t1222: F, t13053: F, t17353: F, t17448: F, t17475: F, t17640: F, t17650: F, t372: F, t44521: F, t44751: F, t5277: F, t5330: F, t5335: F, t57480: F, t58868: F, t58878: F, t58882: F, t58884: F, t59066: F, t59854: F, t68265: F, t68308: F, t68345: F) -> (F, F) {
    let t71200 = t1715 * t3601;
    let t71207 = t3708 * t20816;
    let t71231 = F::cast_from(0.17149607247227894789e-2_f64) * t59066 * t17353 * t13053 * t71200 + F::cast_from(0.57165357490759649296e-3_f64) * t58868 - F::cast_from(0.6351706387862183255e-4_f64) * t44751 + F::cast_from(0.28582678745379824648e-3_f64) * t71207 - F::cast_from(0.28582678745379824648e-3_f64) * t58878 - F::cast_from(0.11433071498151929859e-2_f64) * t44521 * t372 * t5277 * t1121 * t17650 - F::cast_from(0.85748036236139473944e-3_f64) * t59854 * t5330 * t5335 - F::cast_from(0.19055119163586549765e-3_f64) * t58882 - F::cast_from(0.57165357490759649296e-3_f64) * t58884 - F::new(7.0) / F::new(54.0) * t1222 * t17475 * t68345 - F::new(7.0) / F::new(648.0) * t1222 * t17475 * t68265 + F::new(35.0) / F::new(972.0) * t1222 * t57480 * t68308 - F::cast_from(0.28582678745379824648e-3_f64) * t17448 * t17640;
    (t71200, t71231)
}
