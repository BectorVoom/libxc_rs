//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta201 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk970;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk971;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta201<F: Float>(t1534: F, t177: F, t762: F, t162: F, t2611: F, t1469: F, t189: F, t606: F, t2623: F, t2621: F, t2628: F, t2632: F, t4307: F, t4310: F, t4313: F, t4316: F, t4394: F, t4396: F, t4397: F, t225: F, t4376: F, t227: F, t73: F, t1544: F, t853: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t4398, t4400, t4401, t4402, t4403, t4405, t4406, t4407) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk970::<F>(t1534, t177, t762, t162, t2611, t1469, t189, t606, t2623, t2621, t2628, t2632, t4307, t4310, t4313, t4316, t4394, t4396, t4397);
        let (t4409, t4415, t4416) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk971::<F>(t225, t4376, t4407, t227, t73, t1544, t853);
    (t4398, t4400, t4401, t4402, t4403, t4405, t4406, t4409, t4415, t4416)
}
