//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 474/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk474<F: Float>(t213: F, t218: F, t2014: F, t978: F, t211: F, t215: F, t414: F, t690: F, t2026: F, t982: F, t220: F, t694: F, t43: F, t385: F, t991: F, t426: F, t118: F, t632: F, zeta_threshold: F) -> (F, F, F, F) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t2828 = t2014 * t978;
    let t2831 = t215 * t211;
    let t2835 = piecewise3(t214, 0.0, 4.0 / 9.0 * t2828 * t690 + 8.0 / 3.0 * t2831 * t414);
    let t2836 = t2026 * t982;
    let t2839 = t220 * t211;
    let t2843 = piecewise3(t219, 0.0, 4.0 / 9.0 * t2836 * t694 - 8.0 / 3.0 * t2839 * t414);
    let t2845 = (t2835 + t2843) * t43;
    let t2874 = t385 * t991;
    let t2876 = t426 * t991;
    let t2878 = t632 * t118;
    (t2845, t2874, t2876, t2878)
}
