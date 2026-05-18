//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 840/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk840<F: Float>(t13180: F, t144: F, t13084: F, t13085: F, t13090: F, t13137: F, t13143: F, t13148: F, t13152: F, t13154: F, t13157: F, t13162: F, t13168: F, t13173: F, t13177: F, t1901: F, t3281: F, t446: F, t9405: F) -> F {
    let t13181 = t144 * t13180;
    let t13184 = t13084 - F::new(2.0) * t446 * t13085 - F::new(2.0) / F::new(3.0) * t446 * t13090 - t446 * t13137 / F::new(3.0) - F::new(4.0) / F::new(3.0) * t1901 * t13143 + F::new(2.0) / F::new(9.0) * t1901 * t13148 - t13152 + F::new(2.0) / F::new(9.0) * t1901 * t13154 + F::new(2.0) / F::new(9.0) * t1901 * t13157 + F::new(2.0) / F::new(9.0) * t1901 * t13162 - F::new(2.0) / F::new(9.0) * t1901 * t13168 + F::new(2.0) / F::new(27.0) * t9405 - F::new(4.0) / F::new(9.0) * t3281 * t13173 + F::new(2.0) / F::new(3.0) * t446 * t13177 + F::new(4.0) / F::new(3.0) * t446 * t13181;
    t13184
}
