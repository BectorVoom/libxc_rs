//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 204/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk204<F: Float>(t819: F, t821: F, t825: F, t827: F, t31: F) -> (F, F) {
    let t829 = -F::cast_from(0.632975e0_f64) * t819 - F::cast_from(0.29896666666666666667e0_f64) * t821 - F::cast_from(0.1023875e0_f64) * t825 - F::cast_from(0.82156666666666666667e-1_f64) * t827;
    let t830 = F::cast_from(1.0_f64) / t31;
    (t829, t830)
}
