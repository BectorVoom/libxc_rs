//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 614/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk614<F: Float>(t4637: F, t6756: F, t8512: F, t8516: F, t8520: F) -> F {
    let t8522 = t4637 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t6756 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t8512 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t8516 - t8520 / F::cast_from(3.0_f64);
    t8522
}
