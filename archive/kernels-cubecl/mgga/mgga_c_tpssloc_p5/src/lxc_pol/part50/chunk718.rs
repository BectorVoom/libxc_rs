//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 718/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk718<F: Float>(t1059: F, t6800: F, t6799: F, t1049: F, t1948: F, t345: F, t1022: F, t1945: F, t1060: F, t383: F, t6768: F, t1003: F, t1058: F, t1920: F, t1950: F, t1953: F, t353: F, t6680: F, t6687: F, t6783: F, t6787: F, t6790: F, t6797: F) -> (F, F, F, F, F, F, F) {
    let t6801 = t1059 * t6800;
    let t6802 = t6799 * t6801;
    let t6805 = t1948 * t1049;
    let t6806 = t345 * t6805;
    let t6810 = t1945 * t1022;
    let t6811 = t6810 * t1060;
    let t6813 = t383 * t6768;
    let t6815 = -F::cast_from(0.21932454224643019153e-1_f64) * t6680 * t1950 + t6783 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t6787 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t6790 + F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t6802 + F::cast_from(0.82246703342411321825e-2_f64) * t1920 * t6806 + t1003 * t1953 + t1058 * t6811 + t353 * t6813;
    (t6801, t6802, t6805, t6810, t6811, t6813, t6815)
}
