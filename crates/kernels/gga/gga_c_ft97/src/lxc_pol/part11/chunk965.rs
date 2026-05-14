//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 965/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk965<F: Float>(t1882: F, t9816: F, t10026: F, t9993: F, t10046: F, t10041: F, t756: F, t89: F, t9555: F, t2587: F, t8232: F, t8392: F, t9799: F, t9794: F, t9985: F, t10074: F, t10079: F, t10080: F, t10166: F, t1901: F, t1934: F, t2409: F, t242: F, t2459: F, t2599: F, t2600: F, t2606: F, t41403: F, t446: F, t684: F, t724: F, t9787: F, t9793: F, t9983: F) -> (F, F, F, F, F, F, F) {
    let t42785 = t1882 * t9816;
    let t42795 = t1882 * t10026;
    let t42805 = t1882 * t9993;
    let t42807 = t1882 * t10046;
    let t42809 = t1882 * t10041;
    let t42812 = t89 * t9555 * t756;
    let t42819 = t8232 * t2587;
    let t42832 = t8392 * t9799;
    let t42834 = t8392 * t9794;
    let t42836 = t8392 * t9985;
    let t42850 = -4.0 / 9.0 * t446 * t724 * t10166 * t684 - 16.0 / 27.0 * t42819 - 8.0 * t446 * t242 * t41403 - 8.0 / 3.0 * t1901 * t9787 * t9793 + 2.0 / 3.0 * t1901 * t2599 * t2600 * t1934 * t2459 + 8.0 / 9.0 * t42832 + 8.0 / 9.0 * t42834 - 4.0 / 9.0 * t42836 - 4.0 / 3.0 * t1901 * t2599 * t9983 * t2409 + 8.0 / 3.0 * t1901 * t10079 * t10080 * t2409 + 8.0 / 3.0 * t1901 * t2606 * t10074 * t2409;
    (t42785, t42795, t42805, t42807, t42809, t42812, t42850)
}
