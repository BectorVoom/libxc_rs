//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 842/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk842<F: Float>(t12868: F, t5907: F, t3831: F, t458: F, t1364: F, t3593: F, t457: F, t1430: F, t3517: F, t1435: F, t1202: F, t3721: F) -> (F, F, F, F, F, F, F) {
    let t12869 = t5907 * t12868;
    let t12872 = t458 * t3831;
    let t12873 = t3593 * t1364;
    let t12874 = t12872 * t12873;
    let t12875 = t457 * t12874;
    let t12878 = t3517 * t1430;
    let t12880 = t3517 * t1435;
    let t12884 = F::cast_from(1.0_f64) / t3721 / t1202;
    (t12869, t12873, t12874, t12875, t12878, t12880, t12884)
}
