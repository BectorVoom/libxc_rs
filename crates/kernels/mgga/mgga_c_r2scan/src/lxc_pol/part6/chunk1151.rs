//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1151/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1151<F: Float>(t378: F, t5: F, t5991: F, t18783: F, t236: F, t735: F, t4959: F, t736: F, t1754: F, t5234: F, t1751: F, t5231: F, t1422: F, t2036: F, t18911: F, t230: F) -> (F, F, F, F, F, F, F) {
    let t21044 = t5 * t378 * t5991;
    let t21048 = 0.5622597711267568807e-1 * t735 * t18783 * t236;
    let t21050 = t4959 * t5 * t736;
    let t21052 = t5234 * t1754;
    let t21054 = t1751 * t5231;
    let t21056 = t1422 * t2036;
    let t21060 = t18911 * t230;
    (t21044, t21048, t21050, t21052, t21054, t21056, t21060)
}
