//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 970/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk970<F: Float>(t1742: F, t3036: F, t3503: F, t3500: F, t1210: F, t11539: F, t4724: F, t1174: F, t13969: F, t4983: F, t3515: F, t478: F) -> (F, F, F, F, F) {
    let t15501 = t1742 * t3036;
    let t15502 = t3503 * t15501;
    let t15503 = t3500 * t15502;
    let t15506 = t1210 * t15501;
    let t15507 = t3500 * t15506;
    let t15522 = t11539 * t4724;
    let t15524 = t1174 * t15522 / F::cast_from(324.0_f64);
    let t15548 = t13969 * t4983;
    let t15550 = t3515 * t15548 / F::cast_from(2304.0_f64);
    let t15567 = t478 * t1742;
    (t15503, t15507, t15524, t15550, t15567)
}
