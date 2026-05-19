//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1429/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1429<F: Float>(t35204: F, t6734: F, t204: F, t34246: F, t587: F, t2413: F, t26127: F, t34239: F, t6717: F, t6914: F, t10241: F, t1359: F) -> (F, F, F, F, F) {
    let t35206 = F::cast_from(0.21450293971110256001e2_f64) * t35204 * t6734;
    let t35209 = F::cast_from(0.92023022289409799224e1_f64) * t587 * t204 * t34246;
    let t35211 = F::cast_from(0.21450293971110256002e1_f64) * t26127 * t2413;
    let t35214 = F::cast_from(0.12423108009070322895e3_f64) * t6914 * t6717 * t34239;
    let t35215 = t1359 * t10241;
    (t35206, t35209, t35211, t35214, t35215)
}
