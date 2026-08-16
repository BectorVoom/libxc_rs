//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta367 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1322;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta367<F: Float>(t2516: F, t5571: F, t5566: F, t72: F, t757: F, t1320: F, t5567: F, t5569: F, t9395: F, t9398: F, t1353: F, t1448: F) -> (F, F, F, F, F, F, F) {
        let (t13612, t13615, t13620, t13622, t13623, t13624, t13625) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1322::<F>(t2516, t5571, t5566, t72, t757, t1320, t5567, t5569, t9395, t9398, t1353, t1448);
    (t13612, t13615, t13620, t13622, t13623, t13624, t13625)
}
