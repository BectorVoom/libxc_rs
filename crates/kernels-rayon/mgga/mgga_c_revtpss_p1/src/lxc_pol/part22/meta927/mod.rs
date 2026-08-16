//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta927 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3151;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3152;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta927(t17728: f64, t3555: f64, t489: f64, t12772: f64, t17736: f64, t17738: f64, t3623: f64, t53739: f64, t13127: f64, t12865: f64, t3746: f64, t12831: f64, t17395: f64, t12702: f64, t17350: f64, t1263: f64, t372: f64, t5284: f64, t13148: f64, t460: f64, t17261: f64, t17373: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56861, t56867, t56878, t56879, t56888, t56953) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3151(t17728, t3555, t489, t12772, t17736, t17738, t3623, t53739, t13127, t12865, t3746, t12831, t17395);
        let (t56977, t56981, t56997, t57005, t57021) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3152(t12702, t17350, t1263, t372, t5284, t13148, t56878, t17728, t460, t489, t17261, t17373);
    (t56861, t56867, t56878, t56879, t56888, t56953, t56977, t56981, t56997, t57005, t57021)
}
