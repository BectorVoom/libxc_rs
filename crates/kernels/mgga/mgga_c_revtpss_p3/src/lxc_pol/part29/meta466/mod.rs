//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta466<F: Float>(t13625: F, t26405: F, t531: F, t7535: F, t7238: F, t2089: F, t2371: F, t198: F, t206: F, t2070: F) -> (F, F, F, F, F) {
        let (t26406, t26411, t26412, t26415, t26425) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1723::<F>(t13625, t26405, t531, t7535, t7238, t2089, t2371, t198, t206, t2070);
    (t26406, t26411, t26412, t26415, t26425)
}
