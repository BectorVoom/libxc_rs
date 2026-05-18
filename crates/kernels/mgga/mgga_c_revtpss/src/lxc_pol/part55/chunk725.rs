//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 725/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk725<F: Float>(t1227: F, t1238: F, t1252: F, t1266: F, t484: F, t7606: F, t7607: F, t7610: F, t7613: F, t7618: F, t7622: F, t7624: F) -> F {
    let t7627 = t7606 - t7607 * t1227 / F::new(288.0) + F::new(0.42874018118069736972e-3) * t7610 * t484 - F::new(0.42874018118069736972e-3) * t7613 * t1238 + F::new(0.42874018118069736972e-3) * t7618 * t1252 + t7622 - F::new(0.28582678745379824648e-3) * t7624 * t1266;
    t7627
}
