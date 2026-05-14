//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1093/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1093<F: Float>(t1021: F, t19911: F, t19857: F, t28029: F, t26933: F, t29056: F, t29051: F, t92544: F, t28059: F, t5062: F, t19937: F, t7754: F, t19849: F, t26930: F, t19655: F, t3338: F) -> (F, F, F, F, F, F, F, F) {
    let t99962 = t1021 * t19911;
    let t99964 = t28029 * t19857;
    let t99966 = t26933 * t29056;
    let t99968 = t92544 * t29051;
    let t99970 = t28059 * t5062;
    let t99972 = t7754 * t19937;
    let t99974 = t26930 * t19849;
    let t99977 = t7754 * t3338 * t19655;
    (t99962, t99964, t99966, t99968, t99970, t99972, t99974, t99977)
}
