//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 616/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk616<F: Float>(t5828: F, t977: F, t3003: F, t4384: F, t5718: F, t5721: F, t5724: F) -> (F, F) {
    let t5829 = t977 * t5828;
    let t5836 = -t3003 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4384 + t5718 / F::cast_from(18.0_f64) - t5721 / F::cast_from(3.0_f64) + t5724 / F::cast_from(6.0_f64);
    (t5829, t5836)
}
