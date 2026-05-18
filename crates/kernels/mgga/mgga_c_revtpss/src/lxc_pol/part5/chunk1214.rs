//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1214/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1214<F: Float>(t1634: F, t4707: F, t6209: F, t972: F, t6206: F, t3014: F, t6205: F, t4711: F, t11509: F, t6189: F, t15101: F, t4595: F) -> (F, F, F, F, F, F, F) {
    let t19294 = t1634 * t4707;
    let t19297 = t6209 * t972;
    let t19300 = t6206 * t972;
    let t19303 = t6205 * t3014;
    let t19304 = t19303 * t972;
    let t19307 = t4711 * t4707;
    let t19310 = t6189 * t11509;
    let t19311 = t19310 * t972;
    let t19315 = F::new(4.0) * t15101 * t4595;
    (t19294, t19297, t19300, t19304, t19307, t19311, t19315)
}
