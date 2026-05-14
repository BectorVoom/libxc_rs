//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1019/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1019<F: Float>(t31393: F, t795: F, t113: F, t40393: F, t97: F, t40491: F, t986: F, t11554: F, t2850: F, t10776: F, t10810: F, t3115: F, t3295: F, t9540: F, t9517: F, t3308: F, t37965: F, t8821: F) -> (F, F, F, F, F, F, F, F) {
    let t42940 = t31393 * t795;
    let t42945 = t97 * t40393 * t113;
    let t42959 = t40491 * t986;
    let t42966 = t11554 * t2850;
    let t42978 = t10776 * t10810 * t3115;
    let t42980 = t3295 * t9540;
    let t42982 = t3295 * t9517;
    let t42985 = t37965 * t3308 * t8821;
    (t42940, t42945, t42959, t42966, t42978, t42980, t42982, t42985)
}
