//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1090/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1090<F: Float>(t1882: F, t34765: F, t34640: F, t34779: F, t34637: F, t103: F, t11854: F, t138154: F, t138156: F, t138158: F, t138168: F, t138176: F, t138178: F, t138184: F, t138208: F, t145909: F, t1557: F, t1570: F, t1901: F, t23265: F, t25990: F, t26171: F, t28: F, t3188: F, t32325: F, t32366: F, t32602: F, t446: F, t447: F, t452: F, t47399: F, t488: F, t5630: F, t5743: F, t60901: F, t7274: F, t82: F, t89: F, t920: F, t925: F, t979: F) -> F {
    let t146824 = t1882 * t34765;
    let t146826 = t1882 * t34640;
    let t146834 = t1882 * t34779;
    let t146851 = t1882 * t34637;
    let t146858 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t138154 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t138156 - F::cast_from(4.0_f64) * t1901 * t26171 * t5630 * t25990 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t138158 + t138168 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t60901 * t32602 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t138176 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t138178 - t446 * t447 * t32366 * t925 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t146824 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t146826 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t138184 + t446 * t452 * t488 * t32325 * t979 / F::cast_from(3.0_f64) + t138208 + t146834 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t11854 * t23265 * t920 * t5743 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t11854 * t7274 * t1570 * t3188 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t47399 * t7274 * t1557 * t3188 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t146851 + t89 * t28 * t82 * t145909 * t103 / F::cast_from(3.0_f64);
    t146858
}
