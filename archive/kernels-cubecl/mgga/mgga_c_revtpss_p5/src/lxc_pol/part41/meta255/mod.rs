//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta255 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk975;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk976;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk977;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk978;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk979;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk980;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta255<F: Float>(t508: F, t8362: F, t569: F, t1911: F, t2178: F, t1312: F, t2179: F, t2181: F, t4248: F, t651: F, t7732: F, t7889: F, t8353: F, t3: F, param_d: F, t1518: F, t8295: F, t117: F, t1916: F, t1918: F, t2187: F, t2189: F, t572: F, t573: F, t587: F, t65: F, t143: F, t2580: F, t130: F, t2566: F, t700: F, t2584: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t8363, t8367) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk975::<F>(t508, t8362, t569);
        let t8369 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk976::<F>(t1911, t2178);
        let (t8372, t8373) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk977::<F>(t1312, t2179, t2181, t4248, t651, t7732, t7889, t8353, t8363, t8367, t8369, t3);
        let t8377 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk978::<F>(t8372, param_d);
        let (t8383, t8386, t8389, t8779) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk979::<F>(t1518, t8295, t117, t8362, t1916, t1918, t2187, t2189, t572, t573, t8377, t587, t65);
        let (t9275, t9278) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk980::<F>(t143, t2580, t130, t2566, t700, t2584);
    (t8363, t8367, t8369, t8372, t8373, t8377, t8383, t8386, t8389, t8779, t9275, t9278)
}
