//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta564 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1975;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta564<F: Float>(t221: F, t22287: F, t1398: F, t6843: F, t1883: F, t5658: F, t543: F, t73842: F, t73820: F, t6862: F, t13790: F, t23037: F) -> (F, F, F, F, F, F, F, F) {
        let (t74477, t74700, t75012, t75016, t75047, t75051, t75188, t75267) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1975::<F>(t221, t22287, t1398, t6843, t1883, t5658, t543, t73842, t73820, t6862, t13790, t23037);
    (t74477, t74700, t75012, t75016, t75047, t75051, t75188, t75267)
}
