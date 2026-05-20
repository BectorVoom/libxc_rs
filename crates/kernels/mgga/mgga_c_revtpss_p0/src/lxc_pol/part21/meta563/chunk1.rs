//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2260/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2260<F: Float>(t1032: F, t5216: F, t1246: F, t1252: F, t12956: F, t12999: F, t13012: F, t13015: F, t13018: F, t17589: F, t17593: F, t17602: F, t17605: F, t3631: F, t3647: F, t3711: F, t3718: F, t5279: F, t5304: F) -> (F, F) {
    let t17608 = t5216 * t1032;
    let t17609 = t17608 * t1246;
    let t17614 = F::cast_from(0.28582678745379824648e-3_f64) * t3711 * t17589 + t17593 + F::cast_from(0.28582678745379824648e-3_f64) * t12956 * t5279 - t12999 / F::new(432.0) + t13012 / F::new(648.0) - t13015 / F::new(864.0) + t13018 / F::new(648.0) - F::cast_from(0.21437009059034868486e-3_f64) * t3718 * t17602 + F::cast_from(0.15244095330869239812e-2_f64) * t17605 * t3631 + F::cast_from(0.42874018118069736972e-3_f64) * t17609 * t1252 + F::cast_from(0.47637797908966374414e-3_f64) * t3647 * t5304;
    (t17609, t17614)
}
