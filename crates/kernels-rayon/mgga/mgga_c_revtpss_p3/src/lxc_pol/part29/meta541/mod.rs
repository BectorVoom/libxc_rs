//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1875;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta541(t93302: f64, t95854: f64, t25310: f64, t26544: f64, t7064: f64, t95575: f64, t2067: f64, t41117: f64, t26502: f64, t786: f64, t789: f64, t93314: f64, t7407: f64, t93179: f64, t25365: f64, t26506: f64, t25305: f64, t95540: f64, t10115: f64, t2063: f64, t213: f64, t26473: f64, t10982: f64, t2061: f64, t9646: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95855, t95857, t95859, t95862, t95866, t95872) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1875(t93302, t95854, t25310, t26544, t7064, t95575, t2067, t41117, t26502, t786, t789, t93314);
        let (t95876, t95888, t95891, t95893, t95894, t95899) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1876(t7407, t93179, t25365, t26506, t25305, t95540, t10115, t2063, t213, t26473, t10982, t2061, t9646);
    (t95855, t95857, t95859, t95862, t95866, t95872, t95876, t95888, t95891, t95893, t95894, t95899)
}
