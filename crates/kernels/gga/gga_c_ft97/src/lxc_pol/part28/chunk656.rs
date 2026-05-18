//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 656/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk656<F: Float>(t376: F, t6526: F, t89: F, t1307: F, t3291: F, t452: F, t1901: F, t23239: F, t23263: F, t23283: F, t26412: F, t26416: F, t26420: F, t26425: F, t26428: F, t26432: F, t26437: F, t26442: F, t26446: F, t446: F) -> F {
    let t26451 = t89 * t376 * t6526;
    let t26454 = t452 * t3291 * t1307;
    let t26457 = -t23239 / F::new(27.0) + t446 * t26412 / F::new(3.0) + t446 * t26416 / F::new(3.0) + t446 * t26420 / F::new(3.0) + t446 * t26425 / F::new(3.0) + t26428 / F::new(27.0) - F::new(2.0) / F::new(9.0) * t23263 - t446 * t26432 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t1901 * t26437 + F::new(2.0) / F::new(27.0) * t1901 * t26442 - t1901 * t26446 / F::new(9.0) + t23283 / F::new(9.0) - t26451 / F::new(9.0) - t446 * t26454 / F::new(3.0);
    t26457
}
