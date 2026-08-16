//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1773;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1774;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1775;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta404(t13126: f64, t487: f64, t460: f64, t1204: f64, t5462: f64, t3754: f64, t5219: f64, t3566: f64, t488: f64, t1276: f64, t1774: f64, t1209: f64, t1828: f64, t3736: f64, t1811: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17948, t17949, t17955, t17958) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1773(t13126, t487, t460, t1204, t5462, t3754, t5219);
        let t17973 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1774(t3566, t488);
        let (t17974, t17986, t17987, t17995) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1775(t1276, t1774, t1209, t488, t1828, t3736, t1811, t3566);
    (t17948, t17949, t17955, t17958, t17973, t17974, t17986, t17987, t17995)
}
