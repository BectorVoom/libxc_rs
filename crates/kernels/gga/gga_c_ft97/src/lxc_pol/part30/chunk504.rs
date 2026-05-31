//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 504/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk504<F: Float>(t10491: F, t309: F, t2: F, t7640: F, t305: F, t631: F, t7242: F, t798: F, t898: F, t192: F) -> (F, F, F, F) {
    let t10492 = t10491 * t309;
    let t10570 = t7640 * t2;
    let t10631 = F::cast_from(1.0_f64) / t305 / t631 / t898 / t798 / t7242 / F::cast_from(4.0_f64);
    let t10683 = t192 * t7640;
    (t10492, t10570, t10631, t10683)
}
