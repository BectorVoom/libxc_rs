//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1972;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta561<F: Float>(t10308: F, t1466: F, t21661: F, t602: F, t2246: F, t5812: F, t10871: F, t5977: F, t18493: F, t221: F, t18498: F, t6016: F, t836: F) -> (F, F, F, F, F, F, F) {
        let (t60224, t60670, t60673, t61532, t61639, t61725, t61749) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1972::<F>(t10308, t1466, t21661, t602, t2246, t5812, t10871, t5977, t18493, t221, t18498, t6016, t836);
    (t60224, t60670, t60673, t61532, t61639, t61725, t61749)
}
