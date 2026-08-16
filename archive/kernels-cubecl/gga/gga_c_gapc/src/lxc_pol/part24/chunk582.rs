//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 582/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk582<F: Float>(t3439: F, t829: F, t3438: F, t311: F, t896: F, t315: F, t3171: F, t2913: F, t2916: F, t2926: F, t2930: F, t2934: F, t2939: F, t2943: F, t2946: F, t2949: F, t2955: F, t2959: F) -> (F, F, F, F, F, F) {
    let t3440 = t829 * t3439;
    let t3441 = t3438 * t3440;
    let t3443 = t311 * t896;
    let t3444 = t3171 * t315;
    let t3445 = t3443 * t3444;
    let t3477 = -F::cast_from(0.60736713313768998073e-4_f64) * t2913 - F::cast_from(0.20245571104589666024e-4_f64) * t2916 + F::cast_from(0.29524791194193262952e-5_f64) * t2926 - F::cast_from(0.60736713313768998073e-4_f64) * t2930 - F::cast_from(0.43449121406768801913e-4_f64) * t2934 + F::cast_from(0.43449121406768801913e-4_f64) * t2939 + F::cast_from(0.43449121406768801913e-5_f64) * t2943 - F::cast_from(0.77252537861234929801e-5_f64) * t2946 - F::cast_from(0.43449121406768801913e-4_f64) * t2949 - F::cast_from(0.12672660410307567225e-4_f64) * t2955 + F::cast_from(0.43449121406768801913e-4_f64) * t2959;
    (t3440, t3441, t3443, t3444, t3445, t3477)
}
