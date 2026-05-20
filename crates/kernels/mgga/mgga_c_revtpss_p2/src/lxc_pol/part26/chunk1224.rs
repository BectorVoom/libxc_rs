//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1224/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1224<F: Float>(t13226: F, t13250: F, t1456: F, t1458: F, t1464: F, t2111: F, t2118: F, t26704: F, t26743: F, t3: F, t4154: F, t4168: F, t575: F, t7542: F, t7560: F, t95182: F, t95184: F, t95186: F, t95190: F, t95196: F, t96628: F, t96633: F, t96682: F) -> F {
    let tv4rho3sigma1 = t3 * t575 * t96628 + t13226 * t2118 + t13250 * t2111 + F::new(3.0) * t1456 * t26743 + t1458 * t96682 + F::new(3.0) * t1464 * t26704 + F::new(3.0) * t4154 * t7560 + F::new(3.0) * t4168 * t7542 + F::new(6.0) * t95182 + F::new(3.0) * t95184 + F::new(3.0) * t95186 + F::new(6.0) * t95190 + F::new(3.0) * t95196 + F::new(3.0) * t96633;
    tv4rho3sigma1
}
