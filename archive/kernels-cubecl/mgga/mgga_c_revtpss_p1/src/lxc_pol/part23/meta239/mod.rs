//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta239 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1400;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1401;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta239<F: Float>(t159: F, t793: F, t1493: F, t76: F, t1518: F, t94: F, t93: F, t587: F, t65: F, t98: F, t106: F, t143: F, t2580: F, t130: F, t2566: F, t700: F, t2584: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7021, t7719, t7732, t7889, t8779) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1400::<F>(t159, t793, t1493, t76, t1518, t94, t93, t587, t65);
        let (t9163, t9232, t9273, t9274, t9275, t9276, t9278) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1401::<F>(t98, t106, t143, t2580, t130, t2566, t700, t2584);
    (t7021, t7719, t7732, t7889, t8779, t9163, t9232, t9273, t9274, t9275, t9276, t9278)
}
