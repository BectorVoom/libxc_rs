//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1089/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1089<F: Float>(t358: F, t7274: F, t1882: F, t34729: F, t34696: F, t376: F, t89: F, t102524: F, t103472: F, t103510: F, t110: F, t11863: F, t137713: F, t138119: F, t138126: F, t138143: F, t144893: F, t145585: F, t1825: F, t1901: F, t1909: F, t25924: F, t25929: F, t26305: F, t26318: F, t26410: F, t26423: F, t26441: F, t3113: F, t3204: F, t34482: F, t34681: F, t34768: F, t38711: F, t39107: F, t446: F, t452: F, t47548: F, t47666: F, t499: F, t5710: F, t91771: F, t925: F) -> F {
    let t146766 = t7274 * t358;
    let t146775 = t1882 * t34729;
    let t146803 = t89 * t376 * t34696;
    let t146806 = -F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t47666 * t103510 * t26441 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t91771 * t26318 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t102524 * t25924 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t103472 * t25929 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t38711 * t34681 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t11863 * t144893 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t91771 * t26305 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t39107 * t146766 * t3204 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t47548 * t146766 * t3113 + t146775 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t5710 * t26410 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t5710 * t26423 + t138119 + t1901 * t1909 * t137713 * t925 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t138126 + t446 * t452 * t1825 * t34768 / F::cast_from(3.0_f64) - t446 * t452 * t499 * t34482 / F::cast_from(3.0_f64) - t446 * t452 * t110 * t145585 / F::cast_from(3.0_f64) - t146803 / F::cast_from(9.0_f64) - t138143 / F::cast_from(27.0_f64);
    t146806
}
