//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1150/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1150<F: Float>(t167: F, t1717: F, t4670: F, t4836: F, t13677: F, t1727: F, t6313: F, t829: F, t1035: F, t6276: F, t1045: F, t6317: F) -> (F, F, F, F, F, F) {
    let t19347 = t1717 * t167;
    let t19350 = t4836 * t4670;
    let t19353 = t13677 * t1727;
    let t19356 = t6313 * t829;
    let t19359 = t1035 * t6276;
    let t19360 = t19359 * t1045;
    let t19363 = t6317 * t829;
    (t19347, t19350, t19353, t19356, t19360, t19363)
}
