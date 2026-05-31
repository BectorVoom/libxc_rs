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
    let t26457 = -t23239 / F::cast_from(27.0_f64) + t446 * t26412 / F::cast_from(3.0_f64) + t446 * t26416 / F::cast_from(3.0_f64) + t446 * t26420 / F::cast_from(3.0_f64) + t446 * t26425 / F::cast_from(3.0_f64) + t26428 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t23263 - t446 * t26432 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t26437 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t26442 - t1901 * t26446 / F::cast_from(9.0_f64) + t23283 / F::cast_from(9.0_f64) - t26451 / F::cast_from(9.0_f64) - t446 * t26454 / F::cast_from(3.0_f64);
    t26457
}
