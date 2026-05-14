//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 466/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk466<F: Float>(t213: F, t218: F, t2551: F, t2653: F, t2740: F, t2820: F, t978: F, t2018: F, t88: F, t2014: F, t215: F, t982: F, t2026: F, t220: F, t43: F, t385: F, t991: F, t426: F, zeta_threshold: F) -> (F, F, F, F) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t2822 = t2551 + t2653 + t2740 + t2820;
    let t2851 = t978 * t978;
    let t2855 = 2.0 * t88 + 2.0 * t2018;
    let t2859 = piecewise3(t214, 0.0, 4.0 / 9.0 * t2014 * t2851 + 4.0 / 3.0 * t215 * t2855);
    let t2860 = t982 * t982;
    let t2863 = -t2855;
    let t2867 = piecewise3(t219, 0.0, 4.0 / 9.0 * t2026 * t2860 + 4.0 / 3.0 * t220 * t2863);
    let t2869 = (t2859 + t2867) * t43;
    let t2874 = t385 * t991;
    let t2876 = t426 * t991;
    (t2822, t2869, t2874, t2876)
}
