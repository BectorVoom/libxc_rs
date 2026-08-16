//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1823;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta505<F: Float>(t1558: F, t2722: F, t14772: F, t221: F, t2645: F, t14749: F, t14767: F, t4423: F, t836: F, t231: F, t18632: F, t50474: F) -> (F, F, F, F, F, F, F, F) {
        let (t50511, t50538, t50789, t50931, t51436, t51525, t51529, t51570) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1823::<F>(t1558, t2722, t14772, t221, t2645, t14749, t14767, t4423, t836, t231, t18632, t50474);
    (t50511, t50538, t50789, t50931, t51436, t51525, t51529, t51570)
}
