//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 487/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk487<F: Float>(t7679: F, t871: F, t296: F, t193: F, t446: F, t7622: F, t7626: F, t7631: F, t7635: F, t7664: F, t7669: F, t7674: F, t89: F) -> (F, F, F) {
    let t7680 = t871 * t7679;
    let t7681 = t296 * t7680;
    let t7684 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t7622 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t7626 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t7631 - t446 * t7635 / F::cast_from(3.0_f64) + t89 * t193 * t7664 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t7669 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t7674 - t446 * t7681 / F::cast_from(3.0_f64);
    (t7680, t7681, t7684)
}
