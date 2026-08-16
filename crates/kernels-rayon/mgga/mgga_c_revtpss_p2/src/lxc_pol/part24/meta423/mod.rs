//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta423 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1371;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1372;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta423(t12808: f64, t17350: f64, t12865: f64, t12909: f64, t13037: f64, t472: f64, t44372: f64, t44373: f64, t474: f64, t3603: f64, t42871: f64, t482: f64, t675: f64, t828: f64, t3566: f64, t3766: f64, t5330: f64, t1209: f64, t13141: f64, t17708: f64, t371: f64, t481: f64, t9291: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44517, t44521, t44531, t44534, t44535, t44536, t44545) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1371(t12808, t17350, t12865, t12909, t13037, t472, t44372, t44373, t474, t3603, t42871, t482, t675);
        let (t44546, t44551, t44578, t44607) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1372(t44545, t828, t3566, t3766, t5330, t1209, t13141, t17708, t371, t481, t482, t9291);
    (t44517, t44521, t44531, t44534, t44535, t44536, t44546, t44551, t44578, t44607)
}
