//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 698/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk698<F: Float>(t7625: F, t7640: F, t7647: F, t7652: F, t7656: F, t8143: F, t8156: F, t8784: F, t8786: F, t8788: F, t8790: F, t9453: F, t9465: F, t9477: F) -> F {
    let t9484 = -F::cast_from(0.21241846568096930143e-2_f64) * t7625 - t8143 - t7640 + t7647 - t7652 + F::cast_from(0.56448240417072397693e-3_f64) * t7656 + F::cast_from(0.5987120850931904282e-1_f64) * t8784 - F::cast_from(0.11974241701863808564e0_f64) * t8786 - t8156 + F::cast_from(0.79656924630363488034e-3_f64) * t8788 - F::cast_from(0.66380770525302906695e-3_f64) * t8790;
    let t9486 = t9453 + t9465 + t9477 + t9484;
    t9486
}
