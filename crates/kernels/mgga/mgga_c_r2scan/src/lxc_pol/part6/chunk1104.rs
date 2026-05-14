//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1104/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1104<F: Float>(t1422: F, t1510: F, t5002: F, t732: F, t19576: F, t234: F, t446: F, t453: F, t5086: F, t5088: F, t498: F, t6592: F, t1561: F, t2259: F, t792: F, t547: F, t6311: F) -> (F, F, F, F, F, F, F) {
    let t19737 = t1422 * t1510;
    let t19743 = t732 * t5002;
    let t19748 = 0.5848223622634646207e0 * t234 * t446 * t19576 * t453;
    let t19753 = t5086 * t5088;
    let t19758 = t498 * t6592;
    let t19768 = t1561 * t792 * t2259;
    let t19786 = t547 * t6311;
    (t19737, t19743, t19748, t19753, t19758, t19768, t19786)
}
