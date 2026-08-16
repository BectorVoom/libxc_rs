//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1013/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1013<F: Float>(t79: F, t85573: F, t85602: F, t85644: F, t85679: F, t4495: F, t4436: F, t110: F, t12020: F, t16076: F, t16228: F, t1871: F, t1901: F, t1909: F, t3193: F, t38921: F, t39230: F, t4454: F, t446: F, t4462: F, t452: F, t59937: F, t60100: F, t75034: F, t8217: F, t85325: F, t85401: F) -> (F, F, F, F) {
    let t80 = F::cast_from(0.1e-59_f64) < t79;
    let t85682 = piecewise3::<F>(t80, t85573 + t85602 + t85644 + t85679, F::cast_from(0.0_f64));
    let t85687 = t4495 * t4495;
    let t85692 = t4436 * t4436;
    let t85723 = -t446 * t452 * t110 * t85682 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t446 * t1871 * t110 * t85687 + F::cast_from(8.0_f64) * t446 * t38921 * t110 * t85692 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t59937 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t3193 * t60100 * t85325 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t3193 * t12020 * t85401 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t8217 * t16228 * t4462 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t39230 * t16228 * t4454 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t3193 * t16076 * t4454 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t1909 * t16076 * t4462 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t75034;
    (t85682, t85687, t85692, t85723)
}
