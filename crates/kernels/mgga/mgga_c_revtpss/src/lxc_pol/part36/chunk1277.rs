//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1277/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1277<F: Float>(t108187: F, t25895: F, t30095: F, t689: F, t25904: F, t25899: F, t1032: F, t6888: F, t1426: F, t7063: F, t7286: F, t1955: F) -> (F, F, F, F, F, F) {
    let t108188 = t25895 * t108187;
    let t108248 = t30095 * t689;
    let t108249 = t25904 * t108248;
    let t108251 = t25899 * t108248;
    let t108277 = t6888 * t1032;
    let t108278 = t108277 * t1426;
    let t108279 = t7063 * t108278;
    let t108280 = t108279 * t7286;
    let t108282 = t1955 * t108277;
    (t108188, t108249, t108251, t108278, t108280, t108282)
}
