//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 767/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk767<F: Float>(t1056: F, t6334: F, t345: F, t6326: F, t1030: F, t104: F, t1072: F, t3105: F, t3109: F, t3113: F, t4869: F, t4871: F, t4885: F, t4887: F, t6276: F, t1064: F, t6330: F) -> (F, F, F, F) {
    let t6436 = t1056 * t6334;
    let t6439 = t345 * t6326;
    let t6450 = t3105 - t3109 - t3113 - 0.3513e-2 * t104 * t6436 + 0.1171e-2 * t104 * t6439 + 0.11955719325063177623e-1 * t1030 * t6276 - 0.5179538907796306876e-4 * t1072 * t6276 - 0.23911438650126355246e-1 * t4869 + 0.20718155631185227504e-3 * t4871 - 0.26416666666666666666e-2 * t4885 - 0.23526125e-4 * t4887;
    let t6452 = t1064 * t6330;
    (t6436, t6439, t6450, t6452)
}
