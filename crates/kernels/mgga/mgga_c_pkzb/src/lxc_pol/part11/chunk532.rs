//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 532/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk532<F: Float>(t218: F, t219: F, t2774: F, t208: F, t2739: F, t1833: F, t1870: F, t1881: F, t1883: F, t2730: F, t2741: F, t2755: F, t2760: F, t2766: F, t2768: F, t2772: F) -> (F, F, F, F) {
    let t2776 = t218 * t219 * t2774;
    let t2778 = t208 * t2739;
    let t2780 = t218 * t219 * t2778;
    let t2782 = -F::new(0.9494625e0) * t2755 + F::new(0.1898925e1) * t2760 + t1870 - F::cast_from(0.29896666666666666667e0_f64) * t1833 - F::cast_from(0.29896666666666666667e0_f64) * t2730 + F::new(0.8969e0) * t2741 + F::new(0.15358125e0) * t2766 + F::new(0.3071625e0) * t2768 + t1881 - F::cast_from(0.16431333333333333333e0_f64) * t1883 - F::cast_from(0.16431333333333333333e0_f64) * t2772 + F::new(0.24647e0) * t2776 + F::new(0.24647e0) * t2780;
    (t2776, t2778, t2780, t2782)
}
