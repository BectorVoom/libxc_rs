//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1973;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta562<F: Float>(t5977: F, t836: F, t18435: F, t221: F, t61532: F, t6022: F, t23160: F, t1559: F, t4423: F, t14586: F, t231: F, t61749: F) -> (F, F, F, F, F, F, F, F) {
        let (t61756, t62403, t62589, t62593, t62604, t62624, t62628, t62637) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1973::<F>(t5977, t836, t18435, t221, t61532, t6022, t23160, t1559, t4423, t14586, t231, t61749);
    (t61756, t62403, t62589, t62593, t62604, t62624, t62628, t62637)
}
