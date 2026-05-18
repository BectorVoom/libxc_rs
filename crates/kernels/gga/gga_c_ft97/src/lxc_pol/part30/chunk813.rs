//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 813/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk813<F: Float>(t2749: F, t7629: F, t840: F, t681: F, t7664: F, t89: F, t296: F, t34013: F, t7686: F, t824: F, t1901: F, t193: F, t34199: F, t34204: F, t34209: F, t34213: F, t34217: F, t34221: F, t34227: F, t34232: F, t446: F) -> (F, F, F, F, F) {
    let t34236 = t840 * t2749 * t7629;
    let t34241 = t89 * t681 * t7664 / F::new(9.0);
    let t34242 = t296 * t34013;
    let t34246 = t840 * t7686 * t824;
    let t34249 = -F::new(2.0) / F::new(9.0) * t1901 * t34199 + t1901 * t34204 / F::new(9.0) + t1901 * t34209 / F::new(9.0) - F::new(2.0) / F::new(3.0) * t446 * t34213 - F::new(2.0) / F::new(3.0) * t446 * t34217 + t89 * t193 * t34221 / F::new(3.0) + t446 * t34227 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t34232 + F::new(2.0) / F::new(3.0) * t446 * t34236 - t34241 - t446 * t34242 / F::new(3.0) - t446 * t34246 / F::new(3.0);
    (t34236, t34241, t34242, t34246, t34249)
}
