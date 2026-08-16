//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2418;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2419;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2420;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta570<F: Float>(t5962: F, t854: F, t236: F, t807: F, t2476: F, t5966: F, t10717: F, t10719: F, t10723: F, t10746: F, t10749: F, t14780: F, t14783: F, t14817: F, t14820: F, t14823: F, t45: F, t57: F, t5819: F, t633: F, t5825: F, t80: F, t18281: F, t4186: F, t4328: F, t606: F, t766: F, t637: F, t83: F, t4335: F, t770: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18348, t18349, t18352, t18353, t18361) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2418::<F>(t5962, t854, t236, t807, t2476, t5966, t10717, t10719, t10723, t10746, t10749, t14780, t14783, t14817, t14820, t14823);
        let (t18367, t18372, t18378, t18379, t18384, t18390) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2419::<F>(t45, t57, t5819, t633, t5825, t80, t18281, t4186, t4328, t606, t766, t637, t83, t4335, t770, zeta_threshold);
        let t18392 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2420::<F>(t18378, t18390);
    (t18348, t18349, t18352, t18353, t18361, t18367, t18372, t18379, t18384, t18392)
}
