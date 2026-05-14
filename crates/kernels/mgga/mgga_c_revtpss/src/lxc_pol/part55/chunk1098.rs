//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1098/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1098<F: Float>(t26399: F, t7735: F, t28658: F, t27137: F, t7359: F, t28711: F, t8634: F, t2089: F, t28042: F, t651: F, t2322: F, t34028: F, t4254: F, t1518: F, t32575: F, t28043: F) -> (F, F, F, F, F, F, F, F, F) {
    let t128535 = 2.0 * t26399 * t7735;
    let t128537 = 2.0 * t28658 * t7735;
    let t128539 = 2.0 * t7359 * t27137;
    let t128543 = 2.0 * t8634 * t28711;
    let t128552 = 2.0 * t651 * t2089 * t28042;
    let t128554 = 2.0 * t2322 * t34028;
    let t128557 = 2.0 * t4254 * t34028;
    let t128560 = 2.0 * t651 * t32575 * t1518;
    let t128562 = 2.0 * t7359 * t28043;
    (t128535, t128537, t128539, t128543, t128552, t128554, t128557, t128560, t128562)
}
