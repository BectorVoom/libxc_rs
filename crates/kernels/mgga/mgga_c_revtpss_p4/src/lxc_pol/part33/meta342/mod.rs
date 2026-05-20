//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta342 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1354;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta342<F: Float>(t1260: F, t3666: F, t12640: F, t225: F, t480: F, t1236: F, t371: F, t676: F, t1235: F, t12627: F, t1226: F, t697: F) -> (F, F, F, F, F, F, F) {
        let (t12956, t12966, t12967, t12984, t12985, t12987, t13011) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1354::<F>(t1260, t3666, t12640, t225, t480, t1236, t371, t676, t1235, t12627, t1226, t697);
    (t12956, t12966, t12967, t12984, t12985, t12987, t13011)
}
