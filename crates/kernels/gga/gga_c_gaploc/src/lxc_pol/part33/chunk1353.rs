//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1353/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1353<F: Float>(t35198: F, t18970: F, t3381: F, t1: F, t1559: F, t544: F, t986: F, t6734: F, t204: F, t34246: F, t587: F, t2413: F, t26127: F) -> (F, F, F, F, F) {
    let t35199 = F::new(0.14896037479937677779e-1) * t35198;
    let t35200 = t18970 * t3381;
    let t35201 = F::new(0.14896037479937677779e-1) * t35200;
    let t35204 = t544 * t1559 * t986 * t1;
    let t35206 = F::new(0.21450293971110256001e2) * t35204 * t6734;
    let t35209 = F::new(0.92023022289409799224e1) * t587 * t204 * t34246;
    let t35211 = F::new(0.21450293971110256002e1) * t26127 * t2413;
    (t35199, t35201, t35206, t35209, t35211)
}
