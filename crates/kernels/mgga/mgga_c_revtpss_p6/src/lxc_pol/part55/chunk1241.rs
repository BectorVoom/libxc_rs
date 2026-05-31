//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1241/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1241<F: Float>(t2322: F, t34196: F, t4254: F, t1936: F, t28586: F, t651: F, t28653: F, t7003: F, t128334: F, t1937: F, t128336: F, t34251: F, t6993: F) -> (F, F, F, F, F, F, F) {
    let t128485 = F::cast_from(2.0_f64) * t2322 * t34196;
    let t128487 = F::cast_from(2.0_f64) * t4254 * t34196;
    let t128490 = F::cast_from(2.0_f64) * t651 * t28586 * t1936;
    let t128493 = F::cast_from(2.0_f64) * t28653 * t7003;
    let t128495 = F::cast_from(2.0_f64) * t128334 * t1937;
    let t128497 = F::cast_from(2.0_f64) * t128336 * t1937;
    let t128499 = F::cast_from(2.0_f64) * t34251 * t6993;
    (t128485, t128487, t128490, t128493, t128495, t128497, t128499)
}
