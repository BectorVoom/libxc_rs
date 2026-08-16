//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1989;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta550<F: Float>(t1868: F, t4135: F, t116: F, t13424: F, t10871: F, t1558: F, t2722: F, t14772: F, t221: F, t2645: F, t14749: F, t14767: F) -> (F, F, F, F, F, F, F, F) {
        let (t49582, t49686, t50474, t50511, t50538, t50560, t50789, t50931) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1989::<F>(t1868, t4135, t116, t13424, t10871, t1558, t2722, t14772, t221, t2645, t14749, t14767);
    (t49582, t49686, t50474, t50511, t50538, t50560, t50789, t50931)
}
