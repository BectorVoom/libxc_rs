//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 216/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk216<F: Float>(t921: F, t924: F, t261: F, t257: F) -> (F, F, F, F) {
    let t926 = -t921 - F::cast_from(0.17808333333333333333e-1_f64) * t924;
    let t928 = F::cast_from(0.62182e-1_f64) * t926 * t261;
    let t929 = t257 * t257;
    let t930 = F::cast_from(1.0_f64) / t929;
    (t926, t928, t929, t930)
}
