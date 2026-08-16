//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1356;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1357;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta374(t2723: f64, t40262: f64, t10666: f64, t221: f64, t2484: f64, t2485: f64, t2482: f64, t2719: f64, t596: f64, t10852: f64, t2645: f64, t10858: f64, t10863: f64, t10868: f64, t820: f64, t843: f64, t10874: f64, t27: f64, t10872: f64, t10832: f64, t10845: f64, t823: f64, t9948: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40326, t40333, t40337, t40339, t40340, t40345) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1356(t2723, t40262, t10666, t221, t2484, t2485, t2482, t2719, t596, t10852, t2645, t10858, t10863);
        let (t40349, t40355, t40357, t40360) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1357(t10868, t820, t843, t10874, t2482, t27, t10872, t221, t2485, t10832, t10845, t823, t9948);
    (t40326, t40333, t40337, t40339, t40340, t40345, t40349, t40355, t40357, t40360)
}
