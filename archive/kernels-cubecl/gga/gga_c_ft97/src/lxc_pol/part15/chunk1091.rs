//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1091/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1091<F: Float>(t13153: F, t144: F, t167: F, t17198: F, t1901: F, t20684: F, t2210: F, t446: F, t4462: F, t569: F, t574: F, t77821: F, t77823: F, t77868: F, t78438: F, t78565: F, t78573: F, t85538: F, t86868: F, t87097: F) -> F {
    let t87780 = -t446 * t574 * t167 * t86868 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t77821 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t77823 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t446 * t569 * t167 * t85538 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t77868 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t2210 * t17198 * t4462 + F::cast_from(4.0_f64) * t446 * t144 * t87097 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t13153 * t20684 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t78438 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t78565 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t78573;
    t87780
}
