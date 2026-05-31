//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 773/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk773<F: Float>(t33649: F, t33690: F, t33741: F, t33787: F, t7485: F, t771: F, t193: F, t1403: F, t1427: F, t247: F, t33537: F, t33568: F, t33573: F, t33575: F, t33584: F, t33589: F, t33592: F, t33594: F, t33596: F, t33599: F, t33602: F, t33608: F, t33650: F, t33686: F, t5996: F, t6002: F, t719: F, t7443: F, t7487: F, t7558: F) -> (F, F, F, F) {
    let t33789 = t33649 + t33690 + t33741 + t33787;
    let t33792 = t7485 * t771;
    let t33793 = t193 * t33792;
    let t33799 = -t6002 * t33537 / F::cast_from(18.0_f64) + t33568 * t1427 / F::cast_from(6.0_f64) - t33573 - t1403 * t33575 / F::cast_from(3.0_f64) - t5996 * t7443 / F::cast_from(3.0_f64) + t5996 * t7487 / F::cast_from(6.0_f64) + t1403 * t33584 / F::cast_from(6.0_f64) - t33589 + t33592 - t33594 + F::cast_from(4.0_f64) * t33596 - F::cast_from(12.0_f64) * t33599 + F::cast_from(8.0_f64) * t33602 - t247 * t33789 + F::cast_from(8.0_f64) * t33608 + t1403 * t33793 / F::cast_from(6.0_f64) - t719 * t7558 - F::cast_from(2.0_f64) * t33650 + F::cast_from(4.0_f64) * t33686;
    (t33789, t33792, t33793, t33799)
}
