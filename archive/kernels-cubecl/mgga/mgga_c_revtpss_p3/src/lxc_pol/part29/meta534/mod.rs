//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta534<F: Float>(t2435: F, t26447: F, t26485: F, t93342: F, t10509: F, t26481: F, t25387: F, t11015: F, t7388: F, t212: F, t26473: F, t689: F, t780: F) -> (F, F, F, F, F, F) {
        let (t95620, t95624, t95628, t95629, t95632, t95635) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1866::<F>(t2435, t26447, t26485, t93342, t10509, t26481, t25387, t11015, t7388, t212, t26473, t689, t780);
    (t95620, t95624, t95628, t95629, t95632, t95635)
}
