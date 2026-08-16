//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1592/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1592<F: Float>(t114: F, t87050: F, t5876: F, t5883: F, t1519: F, t18245: F, t1843: F, t22578: F, t22633: F, t22634: F, t22639: F, t30138: F, t4248: F, t508: F, t5884: F, t5887: F, t5920: F, t5921: F, t651: F, t6765: F, t75941: F, t7732: F) -> (F, F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t87051 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t87050);
    let t87064 = t5876 * t5883;
    let t87071 = -F::cast_from(8.0_f64) * t1843 * t22633 * t651 - F::cast_from(2.0_f64) * t508 * t651 * t87051 - F::cast_from(12.0_f64) * t5920 * t651 * t6765 - F::cast_from(8.0_f64) * t1519 * t75941 - F::cast_from(24.0_f64) * t18245 * t5887 - F::cast_from(24.0_f64) * t1843 * t22639 - F::cast_from(24.0_f64) * t22578 * t4248 - F::cast_from(24.0_f64) * t22578 * t7732 - F::cast_from(8.0_f64) * t22634 * t4248 - F::cast_from(8.0_f64) * t22634 * t7732 - F::cast_from(24.0_f64) * t30138 * t5921 - F::cast_from(12.0_f64) * t508 * t87064 - F::cast_from(12.0_f64) * t5884 * t6765;
    (t87051, t87064, t87071)
}
