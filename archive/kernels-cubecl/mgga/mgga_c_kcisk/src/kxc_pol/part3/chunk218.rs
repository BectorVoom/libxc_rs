//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 218/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk218<F: Float>(t143: F, t944: F, t146: F, t2: F, t816: F, t819: F, t821: F, t15: F, t818: F) -> (F, F, F, F) {
    let t945 = t143 * t944;
    let t947 = t816 * t146 * t2;
    let t952 = -F::cast_from(0.66066666666666666667e-2_f64) * t819 - F::cast_from(0.41275e-2_f64) * t821;
    let t955 = -t947 * t818 / F::cast_from(12.0_f64) + t15 * t952 / F::cast_from(2.0_f64);
    (t945, t947, t952, t955)
}
