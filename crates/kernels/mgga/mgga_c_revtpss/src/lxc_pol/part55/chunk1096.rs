//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1096/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1096<F: Float>(t2322: F, t34196: F, t4254: F, t1936: F, t28586: F, t651: F, t28653: F, t7003: F, t128334: F, t1937: F, t128336: F, t34251: F, t6993: F, t25082: F, t33183: F, t34301: F) -> (F, F, F, F, F, F, F, F) {
    let t128485 = 2.0 * t2322 * t34196;
    let t128487 = 2.0 * t4254 * t34196;
    let t128490 = 2.0 * t651 * t28586 * t1936;
    let t128493 = 2.0 * t28653 * t7003;
    let t128495 = 2.0 * t128334 * t1937;
    let t128497 = 2.0 * t128336 * t1937;
    let t128499 = 2.0 * t34251 * t6993;
    let t128510 = 3.0 * t25082 * t33183 * t34301;
    (t128485, t128487, t128490, t128493, t128495, t128497, t128499, t128510)
}
