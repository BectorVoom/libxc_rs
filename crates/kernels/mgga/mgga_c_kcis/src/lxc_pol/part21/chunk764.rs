//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 764/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk764<F: Float>(t3132: F, t738: F, t3136: F, t743: F, t3139: F, t733: F, t1080: F, t2475: F, t3116: F, t3119: F, t3124: F, t3127: F, t3142: F, t3145: F, t2635: F, t3160: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10021 = t738 * t3132;
    let t10026 = t743 * t3136;
    let t10028 = t733 * t3139;
    let t10033 = t2475 * t1080;
    let t10035 = t733 * t3116;
    let t10037 = t733 * t3119;
    let t10045 = t738 * t3124;
    let t10048 = t738 * t3127;
    let t10056 = t743 * t3142;
    let t10058 = t743 * t3145;
    let t10087 = t3160 * t2635;
    (t10021, t10026, t10028, t10033, t10035, t10037, t10045, t10048, t10056, t10058, t10087)
}
