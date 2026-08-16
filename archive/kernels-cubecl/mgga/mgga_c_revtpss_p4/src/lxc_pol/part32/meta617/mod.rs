//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1957;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta617<F: Float>(t22295: F, t26028: F, t22299: F, t22093: F, t22098: F, t2018: F, t22129: F, t807: F, t22262: F, t25986: F, t2661: F, t22182: F, t94508: F) -> (F, F, F, F, F, F, F) {
        let (t108543, t108545, t108547, t108549, t108554, t108559, t108562) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1957::<F>(t22295, t26028, t22299, t22093, t22098, t2018, t22129, t807, t22262, t25986, t2661, t22182, t94508);
    (t108543, t108545, t108547, t108549, t108554, t108559, t108562)
}
