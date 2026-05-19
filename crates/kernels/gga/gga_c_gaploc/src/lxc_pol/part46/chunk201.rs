//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 201/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk201<F: Float>(t913: F, t969: F, t825: F, t836: F, t935: F, t568: F, t317: F, t797: F, t813: F, t833: F, t955: F, t960: F, t962: F, t966: F) -> (F, F, F, F, F) {
    let t970 = t969 * t913;
    let t971 = t825 * t970;
    let t973 = t836 * t935;
    let t974 = t568 * t973;
    let t977 = F::cast_from(0.35750489951850426669e0_f64) * t955 * t317 + F::cast_from(0.14896037479937677779e-1_f64) * t960 - F::cast_from(0.35750489951850426669e0_f64) * t797 * t962 - F::cast_from(0.23005755572352449806e1_f64) * t813 * t966 - F::cast_from(0.95857314884801874192e-1_f64) * t971 + F::cast_from(0.23005755572352449806e1_f64) * t833 * t974;
    (t970, t971, t973, t974, t977)
}
