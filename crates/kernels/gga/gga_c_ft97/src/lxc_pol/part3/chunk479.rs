//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 479/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk479<F: Float>(t3653: F, t637: F, t639: F, t2251: F, t2254: F, t2256: F, t2265: F, t3611: F, t3614: F, t3618: F, t3622: F, t3628: F, t3630: F, t3633: F, t3637: F, t3642: F, t631: F) -> (F, F) {
    let t3655 = t637 * t639 * t3653;
    let t3658 = -t2251 - t2254 / F::cast_from(9.0_f64) - t2256 / F::cast_from(3.0_f64) - t3611 / F::cast_from(9.0_f64) + t2265 * t3614 / F::cast_from(18.0_f64) - t2265 * t3618 / F::cast_from(3.0_f64) - t2265 * t3622 / F::cast_from(3.0_f64) + t3628 * t3630 / F::cast_from(3.0_f64) - t3633 / F::cast_from(3.0_f64) - t2265 * t3637 / F::cast_from(3.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t631 * t3642 + t631 * t3655 / F::cast_from(2.0_f64);
    (t3655, t3658)
}
