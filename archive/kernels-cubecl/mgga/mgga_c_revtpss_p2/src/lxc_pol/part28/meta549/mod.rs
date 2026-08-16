//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1998;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta549<F: Float>(t1448: F, t5591: F, t1868: F, t4144: F, t1353: F, t5778: F, t1501: F, t2371: F, t4245: F, t670: F, t2037: F, t4168: F) -> (F, F, F, F, F, F, F) {
        let (t73394, t73488, t75353, t75365, t75485, t75667, t92556) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1998::<F>(t1448, t5591, t1868, t4144, t1353, t5778, t1501, t2371, t4245, t670, t2037, t4168);
    (t73394, t73488, t75353, t75365, t75485, t75667, t92556)
}
