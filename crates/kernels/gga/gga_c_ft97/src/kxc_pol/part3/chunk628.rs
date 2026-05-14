//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 628/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk628<F: Float>(t3622: F, t8675: F, t1068: F, t8640: F, t171: F, t7741: F, t11: F, t41: F, t3630: F, t3637: F, t3614: F, t1075: F, t11171: F, t11169: F, t2253: F, t3655: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12164 = 4.0 / 9.0 * t8675 * t3622;
    let t12165 = t8640 * t1068;
    let t12168 = 1.0 / t171 / t7741;
    let t12169 = t11 * t12168;
    let t12170 = t41 * t12169;
    let t12171 = t12170 * t3630;
    let t12174 = 4.0 / 9.0 * t8675 * t3637;
    let t12190 = 2.0 / 27.0 * t8675 * t3614;
    let t12204 = t8640 * t1075;
    let t12216 = 0.19257444444444444444e0 * t11171;
    let t12217 = 0.6419148148148148148e-1 * t11169;
    let t12240 = 2.0 / 3.0 * t2253 * t3655;
    (t12164, t12165, t12170, t12171, t12174, t12190, t12204, t12216, t12217, t12240)
}
