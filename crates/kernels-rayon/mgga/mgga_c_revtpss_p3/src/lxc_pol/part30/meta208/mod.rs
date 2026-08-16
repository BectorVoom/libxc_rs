//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta208 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk999;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1000;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1001;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta208(t1469: f64, t2852: f64, t606: f64, t2850: f64, t128: f64, t2857: f64, t904: f64, t4186: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4573, t4574, t4575, t4576) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk999(t1469, t2852, t606, t2850, t128);
        let (t4578, t4579, t4580, t4581) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1000(t1469, t2857, t606, t904, t128);
        let t4583 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1001(t4186, t905);
        let (t4584, t4585) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1002(t4583, t904, t128);
    (t4573, t4574, t4575, t4576, t4578, t4579, t4580, t4581, t4583, t4584, t4585)
}
