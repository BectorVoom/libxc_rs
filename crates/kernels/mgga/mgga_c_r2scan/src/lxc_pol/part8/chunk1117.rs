//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1117/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1117<F: Float>(t565: F, t8028: F, t122: F, t625: F, t6412: F, t5149: F, t18783: F, t239: F, t5: F, t4715: F, t753: F, t236: F, t735: F, t1751: F, t5231: F, t1422: F, t2036: F) -> (F, F, F, F, F, F, F, F) {
    let t21003 = t565 * t8028;
    let t21028 = t625 * t6412 * t122;
    let t21029 = t21028 * t5149;
    let t21036 = 1400.0 / 81.0 * t5 * t18783 * t239;
    let t21038 = t5 * t4715 * t753;
    let t21048 = 0.5622597711267568807e-1 * t735 * t18783 * t236;
    let t21054 = t1751 * t5231;
    let t21056 = t1422 * t2036;
    (t21003, t21028, t21029, t21036, t21038, t21048, t21054, t21056)
}
