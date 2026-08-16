//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 991/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk991<F: Float>(t35779: F, t5999: F, t27889: F, t6154: F, t10157: F, t1091: F, t140574: F, t140594: F, t2354: F, t2404: F, t24204: F, t27878: F, t28010: F, t28026: F, t28032: F, t28038: F, t33279: F, t33494: F, t33535: F, t35255: F, t3746: F, t6002: F, t6003: F, t6068: F, t6745: F, t683: F, t7485: F, t9770: F) -> (F, F) {
    let t149884 = t35779 * t5999;
    let t149899 = t6154 * t27889;
    let t149919 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6745 * t33279 - t149884 / F::cast_from(18.0_f64) + t35779 * t6068 / F::cast_from(6.0_f64) + t6002 * t140594 * t28026 / F::cast_from(9.0_f64) + t6002 * t683 * t7485 * t28032 / F::cast_from(9.0_f64) - t6002 * t2404 * t7485 * t28038 / F::cast_from(27.0_f64) - F::cast_from(4.0_f64) * t149899 + t28010 * t2354 * t33535 * t3746 / F::cast_from(9.0_f64) + t24204 * t35255 / F::cast_from(9.0_f64) + t6002 * t9770 * t140574 * t1091 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t28010 * t9770 * t33494 * t3746 + F::cast_from(2.0_f64) * t6002 * t10157 * t6003 * t27878;
    (t149899, t149919)
}
