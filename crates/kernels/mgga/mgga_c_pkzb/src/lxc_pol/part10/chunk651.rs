//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 651/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk651<F: Float>(t218: F, t219: F, t2774: F, t208: F, t2739: F, t1833: F, t1870: F, t1881: F, t1883: F, t2730: F, t2741: F, t2755: F, t2760: F, t2766: F, t2768: F, t2772: F) -> (F, F, F, F) {
    let t2776 = t218 * t219 * t2774;
    let t2778 = t208 * t2739;
    let t2780 = t218 * t219 * t2778;
    let t2782 = -0.9494625e0 * t2755 + 0.1898925e1 * t2760 + t1870 - 0.29896666666666666667e0 * t1833 - 0.29896666666666666667e0 * t2730 + 0.8969e0 * t2741 + 0.15358125e0 * t2766 + 0.3071625e0 * t2768 + t1881 - 0.16431333333333333333e0 * t1883 - 0.16431333333333333333e0 * t2772 + 0.24647e0 * t2776 + 0.24647e0 * t2780;
    (t2776, t2778, t2780, t2782)
}
