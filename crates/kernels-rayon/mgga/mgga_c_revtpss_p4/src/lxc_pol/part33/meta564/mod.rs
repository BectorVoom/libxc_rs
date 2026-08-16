//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta564 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1963;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1964;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta564(t30735: f64, t7637: f64, t2142: f64, t6573: f64, t1769: f64, t8190: f64, t1774: f64, t6563: f64, t1828: f64, t8201: f64, t7652: f64, t1794: f64, t8208: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30736, t30739, t30740, t30743, t30744, t30747) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1963(t30735, t7637, t2142, t6573, t1769, t8190, t1774);
        let (t30748, t30751, t30752, t30757, t30758, t30763) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1964(t30747, t7637, t2142, t6563, t1828, t8201, t7652, t1794, t8208);
    (t30736, t30739, t30740, t30743, t30744, t30747, t30748, t30751, t30752, t30757, t30758, t30763)
}
