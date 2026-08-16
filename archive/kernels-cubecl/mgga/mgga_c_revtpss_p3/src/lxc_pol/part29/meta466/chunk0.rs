//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1723/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1723<F: Float>(t13625: F, t26405: F, t531: F, t7535: F, t7238: F, t2089: F, t2371: F, t198: F, t206: F, t2070: F) -> (F, F, F, F, F) {
    let t26406 = t26405 * t13625;
    let t26411 = t531 * t7535;
    let t26412 = t26411 * t7238;
    let t26415 = t2089 * t2371;
    let t26425 = t198 * t206 * t2070;
    (t26406, t26411, t26412, t26415, t26425)
}
