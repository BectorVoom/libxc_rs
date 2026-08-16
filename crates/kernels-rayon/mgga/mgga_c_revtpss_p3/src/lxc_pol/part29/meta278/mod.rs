//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1144;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1145;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1146;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1147;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta278(t587: f64, t65: f64, t197: f64, t532: f64, t1450: f64, t2106: f64, t143: f64, t2580: f64, t130: f64, t2566: f64, t700: f64, t2584: f64, t121: f64, t131: f64, t141: f64, t22: f64, t2456: f64, t624: f64, t2501: f64, t685: f64, t793: f64, t684: f64, t125: f64, t123: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8779, t8995, t9069, t9274, t9275, t9276) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1144(t587, t65, t197, t532, t1450, t2106, t143, t2580, t130, t2566, t700, t2584);
        let t9278 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1145(t9274, t9276);
        let (t9283, t9285) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1146(t121, t131, t141, t22, t2456, t624);
        let (t9286, t9288) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1147(t2501, t9285, t685, t793);
        let (t9289, t9291, t9292) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1148(t684, t9288, t125, t793, t123);
    (t8779, t8995, t9069, t9275, t9278, t9283, t9285, t9286, t9288, t9289, t9291, t9292)
}
