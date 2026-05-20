//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3393/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3393<F: Float>(t41246: F, t41281: F, t41285: F, t41287: F, t51937: F, t51942: F, t63266: F, t63268: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F) -> F {
    let t63715 = -F::new(0.22076e0) * t51937 + F::new(0.66228e0) * t51942 + t41246 + F::new(0.776775e1) * t63266 - F::new(0.16504875e0) * t63268 + F::cast_from(0.18396666666666666667e0_f64) * t41281 - F::cast_from(0.91983333333333333333e-1_f64) * t41285 - F::cast_from(0.30661111111111111111e-1_f64) * t41287 + F::new(0.12077e1) * t63274 - F::cast_from(0.40256666666666666667e0_f64) * t63276 + F::cast_from(0.13418888888888888889e0_f64) * t63278 - F::cast_from(0.40256666666666666666e0_f64) * t63281 - F::cast_from(0.20128333333333333333e0_f64) * t63285 - F::cast_from(0.33547222222222222222e0_f64) * t63290 + F::new(0.12077e1) * t63293;
    t63715
}
