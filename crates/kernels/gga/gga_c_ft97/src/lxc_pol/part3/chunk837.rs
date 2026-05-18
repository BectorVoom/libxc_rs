//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 837/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk837<F: Float>(t17021: F, t379: F, t9133: F, t12969: F, t3478: F, t12968: F, t1017: F, t2178: F, t3483: F, t13140: F, t13153: F, t3425: F) -> (F, F, F, F) {
    let t17022 = t17021 * t379;
    let t17023 = t9133 * t17022;
    let t17026 = t12969 * t3478;
    let t17027 = t12968 * t17026;
    let t17030 = t2178 * t1017;
    let t17031 = t17030 * t3483;
    let t17032 = t13140 * t17031;
    let t17035 = t13153 * t3425;
    (t17023, t17027, t17032, t17035)
}
