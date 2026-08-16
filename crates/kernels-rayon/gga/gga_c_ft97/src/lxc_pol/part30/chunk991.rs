//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 991/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk991(t35779: f64, t5999: f64, t27889: f64, t6154: f64, t10157: f64, t1091: f64, t140574: f64, t140594: f64, t2354: f64, t2404: f64, t24204: f64, t27878: f64, t28010: f64, t28026: f64, t28032: f64, t28038: f64, t33279: f64, t33494: f64, t33535: f64, t35255: f64, t3746: f64, t6002: f64, t6003: f64, t6068: f64, t6745: f64, t683: f64, t7485: f64, t9770: f64) -> (f64, f64) {
    let t149884 = t35779 * t5999;
    let t149899 = t6154 * t27889;
    let t149919 = -2.0_f64 / 3.0_f64 * t6745 * t33279 - t149884 / 18.0_f64 + t35779 * t6068 / 6.0_f64 + t6002 * t140594 * t28026 / 9.0_f64 + t6002 * t683 * t7485 * t28032 / 9.0_f64 - t6002 * t2404 * t7485 * t28038 / 27.0_f64 - 4.0_f64 * t149899 + t28010 * t2354 * t33535 * t3746 / 9.0_f64 + t24204 * t35255 / 9.0_f64 + t6002 * t9770 * t140574 * t1091 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t28010 * t9770 * t33494 * t3746 + 2.0_f64 * t6002 * t10157 * t6003 * t27878;
    (t149899, t149919)
}
