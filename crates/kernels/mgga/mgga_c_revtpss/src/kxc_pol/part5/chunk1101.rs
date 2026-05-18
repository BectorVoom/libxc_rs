//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1101/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1101<F: Float>(t1065: F, t1668: F, t372: F, t4823: F, t1087: F, t11773: F, t1062: F, t4857: F, t11986: F, t1592: F, t247: F, t1063: F) -> (F, F, F, F, F) {
    let t15690 = t1065 * t1668;
    let t15691 = t372 * t15690;
    let t15696 = t372 * t4823;
    let t15700 = t1087 * t11773;
    let t15707 = t4857 * t1062;
    let t15711 = t247 * t11986 * t1592;
    let t15712 = t1063 * t15711;
    (t15691, t15696, t15700, t15707, t15712)
}
