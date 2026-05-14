//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1346/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1346<F: Float>(t19400: F, t32195: F, t86: F, t10245: F, t468: F, t23830: F, t23837: F, t23901: F, t23903: F, t28048: F, t28051: F, t23915: F, t19394: F, t19405: F, t23829: F, t23835: F, t23906: F, t23910: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t32963 = 0.56968947174242584612e-3 * t19400;
    let t32965 = 0.19751673498613801407e-1 * t32195 * t86;
    let t32966 = t10245 * t468;
    let t32967 = 0.5848223622634646207e0 * t32966;
    let t32968 = 0.97592231702715658578e-1 * t23830;
    let t32969 = 3.0 * t23837;
    let t32970 = 0.48796115851357829289e-1 * t23901;
    let t32971 = 0.10526802520742363173e2 * t23903;
    let t32972 = 0.35089341735807877242e1 * t28048;
    let t32973 = 60.0 * t28051;
    let t32974 = 180.0 * t23915;
    let t32975 = -t19394 - t32963 + t19405 + t32965 - t32967 + t23829 - t32968 + t23835 + t32969 + t32970 + t32971 + t32972 + t23906 + t23910 + t32973 + t32974;
    (t32963, t32965, t32967, t32968, t32969, t32970, t32971, t32972, t32973, t32974, t32975)
}
