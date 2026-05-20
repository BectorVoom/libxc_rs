//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta518<F: Float>(t47671: F, t198: F, t775: F, t2246: F, t4171: F, t10308: F, t1466: F, t21661: F, t602: F, t5812: F, t10871: F, t5977: F) -> (F, F, F, F, F, F, F) {
        let (t47672, t50080, t60221, t60224, t60670, t60673, t61532) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1821::<F>(t47671, t198, t775, t2246, t4171, t10308, t1466, t21661, t602, t5812, t10871, t5977);
    (t47672, t50080, t60221, t60224, t60670, t60673, t61532)
}
