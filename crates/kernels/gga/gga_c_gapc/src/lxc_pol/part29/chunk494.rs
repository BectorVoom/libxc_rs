//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 494/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk494<F: Float>(t218: F, t211: F, t220: F, t2836: F, t414: F, t694: F, t2835: F, t43: F, t385: F, t991: F, t426: F, t118: F, t632: F, zeta_threshold: F) -> (F, F, F, F) {
    let t219 = t218 <= zeta_threshold;
    let t2839 = t220 * t211;
    let t2843 = piecewise3::<f64>(t219, F::new(0.0), F::new(4.0) / F::new(9.0) * t2836 * t694 - F::new(8.0) / F::new(3.0) * t2839 * t414);
    let t2845 = (t2835 + t2843) * t43;
    let t2874 = t385 * t991;
    let t2876 = t426 * t991;
    let t2878 = t632 * t118;
    (t2845, t2874, t2876, t2878)
}
