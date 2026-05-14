//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1037/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1037<F: Float>(t1027: F, t6317: F, t6353: F, t6307: F, t829: F, t1035: F, t6272: F, t1045: F, t167: F, t1717: F, t4670: F, t4836: F, t13677: F, t1727: F, t6313: F, t6276: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19334 = t1027 * t6317;
    let t19336 = t1027 * t6353;
    let t19340 = t6307 * t829;
    let t19343 = t1035 * t6272;
    let t19344 = t19343 * t1045;
    let t19347 = t1717 * t167;
    let t19350 = t4836 * t4670;
    let t19353 = t13677 * t1727;
    let t19356 = t6313 * t829;
    let t19359 = t1035 * t6276;
    (t19334, t19336, t19340, t19344, t19347, t19350, t19353, t19356, t19359)
}
