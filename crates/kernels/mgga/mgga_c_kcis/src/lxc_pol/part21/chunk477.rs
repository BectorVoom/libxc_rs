//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 477/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk477<F: Float>(t104: F, t111: F, t3105: F, t3109: F, t3113: F, t3114: F, t3116: F, t3119: F, t3122: F, t3124: F, t3127: F, t3130: F, t3132: F, t1079: F, t2850: F, t1056: F) -> (F, F, F) {
    let t3135 = t3105 - t3109 - t3113 + 0.9368e-2 * t3114 - 0.3513e-2 * t104 * t3116 + 0.1171e-2 * t104 * t3119 - 0.26416666666666666666e-2 * t3122 + 0.7925e-3 * t111 * t3124 - 0.52833333333333333333e-3 * t111 * t3127 - 0.23526125e-4 * t3130 - 0.1585e-2 * t111 * t3132;
    let t3136 = t1079 * t2850;
    let t3139 = t1056 * t2850;
    (t3135, t3136, t3139)
}
