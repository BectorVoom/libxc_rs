//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1648;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1649;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1650;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1651;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1652;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1653;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta438<F: Float>(t5053: F, t689: F, t5057: F, t12256: F, t1469: F, t2251: F, t12305: F, t128: F, t12268: F, t3360: F, t3362: F, t4186: F, t606: F, t2258: F, t5046: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t16710 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1648::<F>(t5053, t689);
        let (t16711, t16712) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1649::<F>(t16710, t5057, t689);
        let (t16713, t16715, t16717) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1650::<F>(t16712, t12256, t1469, t2251, t12305, t128);
        let (t16720, t16722) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1651::<F>(t12268, t1469, t2251, t3360, t128);
        let (t16725, t16727) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1652::<F>(t3362, t4186, t606, t3360, t128);
        let (t16729, t16731) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1653::<F>(t2258, t5046, t3360, t128);
    (t16710, t16711, t16712, t16713, t16715, t16717, t16720, t16722, t16725, t16727, t16729, t16731)
}
