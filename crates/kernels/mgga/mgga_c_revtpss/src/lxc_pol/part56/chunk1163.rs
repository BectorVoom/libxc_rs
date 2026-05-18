//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1163/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1163<F: Float>(t27060: F, t7735: F, t29432: F, t27137: F, t7586: F, t29427: F, t7003: F, t1518: F, t7583: F, t1937: F, t2126: F, t4292: F) -> (F, F, F, F, F, F, F) {
    let t129459 = t27060 * t7735;
    let t129461 = t29432 * t7735;
    let t129463 = t7586 * t27137;
    let t129465 = t29427 * t7003;
    let t129467 = t7583 * t1518;
    let t129468 = t129467 * t1937;
    let t129470 = t2126 * t4292;
    (t129459, t129461, t129463, t129465, t129467, t129468, t129470)
}
