//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 928/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk928<F: Float>(t38052: F, t82: F, t358: F, t492: F, t1820: F, t363: F, t110: F, t11854: F, t12045: F, t1580: F, t1647: F, t1853: F, t1866: F, t1901: F, t1909: F, t1910: F, t3194: F, t379: F, t38053: F, t38057: F, t38071: F, t38942: F, t39228: F, t39230: F, t446: F, t447: F, t499: F, t7955: F, t8367: F, t8417: F, t8419: F, t8577: F) -> F {
    let t39243 = t38052 * t82;
    let t39252 = t492 * t358;
    let t39253 = t363 * t1820;
    let t39267 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t1909 * t1910 * t1580 * t1820 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t1909 * t12045 * t1580 * t1853 + F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t39228 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t39230 * t3194 * t38942 - F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t446 * t8577 * t499 * t7955 - t446 * t447 * t110 * t38057 / F::cast_from(9.0_f64) - F::cast_from(80.0_f64) / F::cast_from(243.0_f64) * t446 * t39243 * t110 * t38053 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t446 * t1866 * t110 * t38071 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t11854 * t39252 * t39253 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t1909 * t8417 * t8419 * t379 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t1909 * t8367 * t1647;
    t39267
}
