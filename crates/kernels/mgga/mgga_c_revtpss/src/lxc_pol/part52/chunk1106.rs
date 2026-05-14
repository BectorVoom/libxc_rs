//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1106/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1106<F: Float>(t28653: F, t7003: F, t128334: F, t1937: F, t128336: F, t34251: F, t6993: F, t25082: F, t33183: F, t34301: F, t22496: F, t37318: F, t128353: F, t2056: F, t128355: F, t34258: F, t7367: F) -> (F, F, F, F, F, F, F, F, F) {
    let t128493 = 2.0 * t28653 * t7003;
    let t128495 = 2.0 * t128334 * t1937;
    let t128497 = 2.0 * t128336 * t1937;
    let t128499 = 2.0 * t34251 * t6993;
    let t128510 = 3.0 * t25082 * t33183 * t34301;
    let t128513 = 3.0 * t25082 * t37318 * t22496;
    let t128517 = 2.0 * t128353 * t2056;
    let t128519 = 2.0 * t128355 * t2056;
    let t128521 = 2.0 * t34258 * t7367;
    (t128493, t128495, t128497, t128499, t128510, t128513, t128517, t128519, t128521)
}
