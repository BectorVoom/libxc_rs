//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2290;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta581(t17974: f64, t3575: f64, t17807: f64, t225: f64, t494: f64, t1209: f64, t488: f64, t1828: f64, t3736: f64, t3790: f64, t3737: f64, t1811: f64, t3566: f64, t3584: f64, t1277: f64, t1210: f64, t12654: f64, t1271: f64, t1274: f64, t17964: f64, t17968: f64, t17973: f64, t1829: f64, t3556: f64, t3569: f64, t3572: f64, t3576: f64, t3739: f64, t460: f64, t5216: f64, t5220: f64, t5225: f64, t5237: f64, t5246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17975, t17979, t17986) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2290(t17974, t3575, t17807, t225, t494, t1209, t488);
        let (t17987, t17988, t17992, t17995, t17999, t18004) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2291(t1828, t3736, t3575, t3790, t3737, t1811, t3566, t3584, t1277, t1210, t12654, t1271, t1274, t17964, t17968, t17973, t17975, t17979, t17986, t1829, t3556, t3569, t3572, t3576, t3739, t460, t5216, t5220, t5225, t5237, t5246);
    (t17975, t17979, t17986, t17987, t17988, t17992, t17995, t17999, t18004)
}
