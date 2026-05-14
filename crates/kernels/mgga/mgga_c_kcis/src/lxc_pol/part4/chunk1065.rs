//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1065/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1065<F: Float>(t187: F, t4731: F, t1684: F, t3005: F, t1831: F, t3551: F, t1835: F, t3006: F, t1219: F, t5234: F, t3569: F, t5237: F, t10893: F, t10898: F, t10936: F, t13798: F, t13801: F, t13805: F, t13974: F, t3550: F, t3575: F, t3586: F, t3592: F, t5216: F, t5238: F) -> (F, F) {
    let t15296 = t187 * t4731;
    let t15304 = t1684 * t3005;
    let t15307 = t1831 * t3551;
    let t15310 = t1835 * t3006;
    let t15317 = t5234 * t1219;
    let t15320 = t1831 * t3569;
    let t15323 = t5237 * t3551;
    let t15326 = -0.19751789702565206229e-1 * t13974 + t13798 + t13801 - t13805 - 0.11696446794910408142e1 * t15304 * t3586 + 6.0 * t3575 * t15307 + 0.35089340384731224426e1 * t3592 * t15310 - 4.0 * t10936 * t5216 + 0.64329366355741395948e2 * t10893 * t5238 - 4.0 * t3550 * t15317 - 2.0 * t3550 * t15320 - 0.19298809906722418785e3 * t10898 * t15323;
    (t15296, t15326)
}
