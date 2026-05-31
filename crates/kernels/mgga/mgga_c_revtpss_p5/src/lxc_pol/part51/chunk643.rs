//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 643/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk643<F: Float>(t1937: F, t2322: F, t4254: F, t1310: F, t1936: F) -> (F, F, F) {
    let t6990 = F::cast_from(2.0_f64) * t2322 * t1937;
    let t6992 = F::cast_from(2.0_f64) * t4254 * t1937;
    let t6993 = t1310 * t1936;
    (t6990, t6992, t6993)
}
