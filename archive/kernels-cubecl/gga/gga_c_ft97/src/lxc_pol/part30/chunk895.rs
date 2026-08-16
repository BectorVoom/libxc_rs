//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 895/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk895<F: Float>(t36160: F, t36202: F, t36239: F, t36271: F, t1218: F, t1466: F, t1479: F, t301: F, t34267: F, t34276: F, t34283: F, t34335: F, t36049: F, t36057: F, t36061: F, t36064: F, t36066: F, t36069: F, t36093: F, t36097: F, t36101: F, t36105: F, t36109: F, t6216: F, t6963: F, t7024: F, t7581: F, t7614: F, t7684: F) -> (F, F) {
    let t36273 = t36160 + t36202 + t36239 + t36271;
    let t36275 = t1466 * t36049 / F::cast_from(3.0_f64) + t7581 * t7024 / F::cast_from(6.0_f64) + t6963 * t7614 / F::cast_from(6.0_f64) - t1466 * t36057 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) * t36061 - F::cast_from(12.0_f64) * t36064 + F::cast_from(8.0_f64) * t36066 + F::cast_from(8.0_f64) * t36069 + t36093 * t1479 / F::cast_from(6.0_f64) + t1466 * t36097 / F::cast_from(3.0_f64) - t34267 - t1218 * t7684 - F::cast_from(2.0_f64) * t36101 + t1466 * t36105 / F::cast_from(6.0_f64) - t6216 * t36109 / F::cast_from(9.0_f64) - t34276 + t34283 - t34335 - t301 * t36273;
    (t36273, t36275)
}
