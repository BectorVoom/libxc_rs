//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1909;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta581<F: Float>(t5722: F, t96576: F, t28780: F, t94890: F, t2435: F, t28825: F, t14079: F, t26265: F, t98108: F, t98128: F, t98130: F, t98144: F) -> (F, F, F, F, F, F, F, F) {
        let (t102453, t102458, t102462, t102465, t102468, t102477, t102478, t102487) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1909::<F>(t5722, t96576, t28780, t94890, t2435, t28825, t14079, t26265, t98108, t98128, t98130, t98144);
    (t102453, t102458, t102462, t102465, t102468, t102477, t102478, t102487)
}
