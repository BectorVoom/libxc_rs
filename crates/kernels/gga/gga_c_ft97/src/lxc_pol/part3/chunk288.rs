//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 288/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk288<F: Float>(t238: F, t1127: F, t695: F, t1097: F, t1111: F, t1115: F, t224: F, t678: F) -> F {
    let t239 = F::new(0.1e-59) < t238;
    let t1128 = t695 * t1127;
    let t1131 = piecewise3::<F>(t239, -F::cast_from(0.11627450473218896e-1_f64) * t678 * t1097 + F::new(2.0) * t1115 + F::cast_from(0.59273806478425129876e-2_f64) * t238 * t1111 - t224 * t1128, F::new(0.0));
    t1131
}
