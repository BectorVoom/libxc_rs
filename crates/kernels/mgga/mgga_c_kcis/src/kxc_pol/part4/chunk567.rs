//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 567/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk567<F: Float>(t2918: F, t2919: F, t2922: F, t2925: F, t2928: F, t261: F, t926: F, t930: F, t951: F, t257: F, t929: F, t244: F) -> (F, F, F, F, F, F) {
    let t2930 = t2918 + F::cast_from(0.11872222222222222222e-1_f64) * t2919 - F::cast_from(0.11872222222222222222e-1_f64) * t2922 + F::cast_from(0.35616666666666666666e-1_f64) * t2925 - F::cast_from(0.17808333333333333333e-1_f64) * t2928;
    let t2932 = F::new(0.62182e-1) * t2930 * t261;
    let t2933 = t926 * t930;
    let t2935 = F::new(2.0) * t2933 * t951;
    let t2936 = t929 * t257;
    let t2937 = F::new(1.0) / t2936;
    let t2938 = t244 * t2937;
    (t2930, t2932, t2933, t2935, t2937, t2938)
}
