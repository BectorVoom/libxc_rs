//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 828/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk828<F: Float>(t144: F, t32995: F, t167: F, t32869: F, t574: F, t1882: F, t7409: F, t376: F, t7392: F, t89: F, t1901: F, t33176: F, t33180: F, t33184: F, t33188: F, t33193: F, t33196: F, t33200: F, t33204: F, t446: F) -> (F, F, F, F, F) {
    let t33207 = t144 * t32995;
    let t33211 = t574 * t167 * t32869;
    let t33215 = t1882 * t7409 / F::new(9.0);
    let t33218 = t89 * t376 * t7392 / F::new(9.0);
    let t33219 = F::new(4.0) / F::new(3.0) * t446 * t33176 + F::new(4.0) / F::new(3.0) * t446 * t33180 - t446 * t33184 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t446 * t33188 - F::new(2.0) / F::new(9.0) * t1901 * t33193 + F::new(2.0) / F::new(9.0) * t1901 * t33196 - F::new(4.0) / F::new(3.0) * t1901 * t33200 - F::new(4.0) / F::new(3.0) * t1901 * t33204 + F::new(2.0) / F::new(3.0) * t446 * t33207 - t446 * t33211 / F::new(3.0) + t33215 - t33218;
    (t33207, t33211, t33215, t33218, t33219)
}
