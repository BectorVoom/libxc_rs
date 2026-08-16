//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 972/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk972(t140: f64, t132: f64, t133: f64, t139: f64, t1683: f64, t1698: f64, t1992: f64, t2043: f64, t2058: f64, t2072: f64, t39802: f64, t39803: f64, t39807: f64, t39813: f64, t39818: f64, t39824: f64, t39828: f64, t40051: f64, t40099: f64, t40128: f64, t40164: f64, t40193: f64, t40226: f64, t40258: f64, t5785: f64, t8825: f64, t8852: f64, t8859: f64, t8895: f64) -> f64 {
    let t141 = 0.1e-59_f64 < t140;
    let t40262 = piecewise3(t141, -12.0_f64 * t1992 * t2072 + 24.0_f64 * t133 * t39802 * t39803 + 6.0_f64 * t133 * t2058 * t39807 + 0.45910941751869106328e2_f64 * t8895 * t1683 + 6.0_f64 * t39813 * t132 * t139 + 0.17516464591774387197e2_f64 * t8859 * t39818 - 0.87582322958871935983e1_f64 * t8852 * t39818 - 0.22445349300913785316e3_f64 * t5785 * t39824 + 0.11222674650456892658e3_f64 * t2043 * t39828 - 0.35032929183548774393e2_f64 * t8825 * t1698 + t40051 + t40099 + t40128 + t40164 + t40193 + t40226 + t40258, 0.0_f64);
    t40262
}
