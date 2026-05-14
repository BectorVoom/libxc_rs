//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1181/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1181<F: Float>(t11772: F, t11775: F, t11778: F, t11780: F, t11792: F, t11825: F, t19995: F, t19996: F, t19997: F, t19998: F, t6592: F, t694: F, t839: F, t1662: F, t105: F, t11828: F, t11834: F, t11837: F, t19999: F, t20000: F, t20001: F, t20002: F, t20003: F, t20004: F, t20005: F, t6583: F, t814: F, t96: F) -> (F, F) {
    let t24578 = 3.0 * t6592 * t694 * t839 - t11772 - t11775 + t11778 - t11780 + t11792 + t11825 - t19995 - t19996 + t19997 - t19998;
    let t24582 = t1662 * t1662;
    let t24587 = -2.0 * t105 * t24582 * t814 * t96 - 3.0 * t6583 * t694 * t839 + t11828 - t11834 + t11837 - t19999 - t20000 - t20001 - t20002 - t20003 - t20004 - t20005;
    (t24578, t24587)
}
