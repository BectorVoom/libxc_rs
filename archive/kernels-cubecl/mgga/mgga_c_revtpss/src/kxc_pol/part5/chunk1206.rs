//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1206/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1206<F: Float>(t3022: F, t6219: F, t6223: F, t2986: F, t6205: F, t974: F, t981: F, t4708: F, t4724: F, t3336: F, t6396: F, t6184: F, t964: F) -> (F, F, F, F, F, F) {
    let t19143 = F::cast_from(0.11696447245269292414e1_f64) * t3022 * t6219;
    let t19145 = F::cast_from(0.5848223622634646207e0_f64) * t3022 * t6223;
    let t19146 = t2986 * t6205;
    let t19147 = t19146 * t974;
    let t19149 = F::cast_from(0.11696447245269292414e1_f64) * t981 * t19147;
    let t19150 = t4724 * t4708;
    let t19152 = F::cast_from(0.23392894490538584828e1_f64) * t981 * t19150;
    let t19153 = t6396 * t3336;
    let t19156 = t6184 * t964;
    (t19143, t19145, t19149, t19152, t19153, t19156)
}
