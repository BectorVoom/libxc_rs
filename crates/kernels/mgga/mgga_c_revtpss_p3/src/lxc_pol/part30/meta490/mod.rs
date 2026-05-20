//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1835;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1836;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1837;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1838;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta490<F: Float>(t26009: F, t2736: F, t2689: F, t7256: F, t2018: F, t3951: F, t807: F, t1941: F, t550: F, t3946: F, t1389: F, t25240: F, t3964: F, t7262: F, t820: F, t843: F, t1401: F, t241: F, t3940: F, t3926: F, t7264: F, t26003: F, t26006: F, t26007: F, t25970: F, t25974: F, t25976: F, t25980: F, t25984: F, t25989: F, t25990: F, t25992: F, t25994: F, t25998: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t26011, t26013, t26014, t26015, t26016, t26018, t26021) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1835::<F>(t26009, t2736, t2689, t7256, t2018, t3951, t807, t1941, t550, t3946, t1389, t25240, t3964);
        let (t26022, t26024) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1836::<F>(t26021, t7262, t820, t843);
        let (t26025, t26028) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1837::<F>(t1401, t26024, t241, t7262, t820);
        let t26033 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1838::<F>(t26028, t3940, t3926, t7264, t26003, t26006, t26007, t26011, t26013, t26016, t26018, t26022, t26025);
        let t26034 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1839::<F>(t25970, t25974, t25976, t25980, t25984, t25989, t25990, t25992, t25994, t25998, t26033);
    (t26011, t26013, t26014, t26015, t26022, t26024, t26025, t26028, t26034)
}
