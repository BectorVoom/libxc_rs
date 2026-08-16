//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1374/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1374<F: Float>(t21876: F, t655: F, t10201: F, t10202: F, t13448: F, t13451: F, t13453: F, t21818: F, t21821: F, t21824: F, t21827: F, t21830: F, t69: F) -> F {
    let t21877 = t655 * t21876;
    let t21880 = -t10201 - F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t10202 - F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t13448 - t13451 + t13453 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t21818 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t69 * t21821 + t69 * t21824 / F::cast_from(2.0_f64) + t21827 / F::cast_from(3.0_f64) + t69 * t21830 / F::cast_from(4.0_f64) - t69 * t21877 / F::cast_from(8.0_f64);
    t21880
}
