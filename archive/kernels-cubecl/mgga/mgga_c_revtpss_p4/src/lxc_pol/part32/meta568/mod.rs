//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta568<F: Float>(t101788: F, t6960: F, t26205: F, t7709: F, t28640: F, t6963: F, t28141: F, t7349: F, t101226: F, t2047: F, t7706: F, t95283: F) -> (F, F, F, F, F, F) {
        let (t101790, t101793, t101811, t101820, t101850, t101870) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1892::<F>(t101788, t6960, t26205, t7709, t28640, t6963, t28141, t7349, t101226, t2047, t7706, t95283);
    (t101790, t101793, t101811, t101820, t101850, t101870)
}
