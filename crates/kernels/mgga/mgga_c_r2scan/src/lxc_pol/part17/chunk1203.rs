//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1203/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1203<F: Float>(t481: F, t9573: F, t2847: F, t3582: F, t2333: F, t3016: F, t795: F, t12043: F, t12721: F, t12723: F, t12726: F, t12728: F, t12730: F, t12733: F, t41116: F, t41117: F, t41118: F, t41119: F, t41120: F, t41121: F, t41122: F, t41123: F) -> (F, F, F, F) {
    let t43959 = t9573 * t481;
    let t43979 = t3582 * t2847;
    let t43983 = t2333 * t3016;
    let t43984 = t43983 * t795;
    let t44008 = -t12721 + t41116 - t41117 + t41118 - t41119 - t41120 + t41121 + t12723 + t12726 + t12728 + t41122 + t12043 + t12730 + t12733 - t41123;
    (t43959, t43979, t43984, t44008)
}
