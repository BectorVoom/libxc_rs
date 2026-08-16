//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1176/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1176<F: Float>(t2990: F, t4531: F, t2824: F, t3003: F, t4384: F, t4387: F, t4390: F, t4393: F) -> (F, F) {
    let t4532 = t4531 * t2990;
    let t4540 = -t3003 - t2824 / F::cast_from(9.0_f64) - t4384 / F::cast_from(9.0_f64) + t4387 / F::cast_from(18.0_f64) - t4390 / F::cast_from(3.0_f64) + t4393 / F::cast_from(6.0_f64);
    (t4532, t4540)
}
