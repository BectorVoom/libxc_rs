//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta773 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2859;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2860;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta773(t3367: f64, t3603: f64, t2251: f64, t12839: f64, t2258: f64, t3555: f64, t3766: f64, t5330: f64, t1209: f64, t13147: f64, t17708: f64, t12854: f64, t17350: f64, t12808: f64, t12865: f64, t12909: f64, t13037: f64, t472: f64, t482: f64, t675: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44458, t44459, t44466, t44484, t44500, t44510) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2859(t3367, t3603, t2251, t12839, t2258, t3555, t3766, t5330, t1209, t13147, t17708, t12854, t17350);
        let (t44517, t44521, t44531, t44535, t44545, t44546) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2860(t12808, t17350, t12865, t12909, t13037, t472, t3603, t482, t675, t828);
    (t44458, t44459, t44466, t44484, t44500, t44510, t44517, t44521, t44531, t44535, t44545, t44546)
}
