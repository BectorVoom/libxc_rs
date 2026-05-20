//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2766/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2766<F: Float>(t1412: F, t6861: F, t2661: F, t3938: F, t3992: F, t5608: F, t5659: F, t1399: F, t22025: F, t22212: F, t2496: F, t1317: F, t22193: F) -> (F, F, F, F, F, F) {
    let t74026 = t1412 * t6861;
    let t74029 = t2661 * t3992 * t74026 * t3938;
    let t74033 = t2661 * t3992 * t5608 * t5659;
    let t74037 = t2661 * t3992 * t22025 * t1399;
    let t74106 = t22212 * t2496;
    let t74111 = t1317 * t22193;
    (t74026, t74029, t74033, t74037, t74106, t74111)
}
