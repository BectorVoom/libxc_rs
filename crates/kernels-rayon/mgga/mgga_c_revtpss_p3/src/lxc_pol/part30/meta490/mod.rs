//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1835;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1836;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1837;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1838;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta490(t26009: f64, t2736: f64, t2689: f64, t7256: f64, t2018: f64, t3951: f64, t807: f64, t1941: f64, t550: f64, t3946: f64, t1389: f64, t25240: f64, t3964: f64, t7262: f64, t820: f64, t843: f64, t1401: f64, t241: f64, t3940: f64, t3926: f64, t7264: f64, t26003: f64, t26006: f64, t26007: f64, t25970: f64, t25974: f64, t25976: f64, t25980: f64, t25984: f64, t25989: f64, t25990: f64, t25992: f64, t25994: f64, t25998: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26011, t26013, t26014, t26015, t26016, t26018, t26021) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1835(t26009, t2736, t2689, t7256, t2018, t3951, t807, t1941, t550, t3946, t1389, t25240, t3964);
        let (t26022, t26024) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1836(t26021, t7262, t820, t843);
        let (t26025, t26028) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1837(t1401, t26024, t241, t7262, t820);
        let t26033 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1838(t26028, t3940, t3926, t7264, t26003, t26006, t26007, t26011, t26013, t26016, t26018, t26022, t26025);
        let t26034 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1839(t25970, t25974, t25976, t25980, t25984, t25989, t25990, t25992, t25994, t25998, t26033);
    (t26011, t26013, t26014, t26015, t26022, t26024, t26025, t26028, t26034)
}
