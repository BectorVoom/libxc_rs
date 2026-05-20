//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2835/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2835<F: Float>(t141: F, t2908: F, t51905: F, t15183: F, t698: F, t15172: F, t2439: F, t4625: F, t4622: F, t15186: F, t51890: F, t51892: F, t51894: F, t51896: F, t51899: F, t51902: F) -> (F, F, F, F, F, F, F) {
    let t51907 = t141 * t2908 * t51905;
    let t51909 = t698 * t15183;
    let t51911 = t698 * t15172;
    let t51913 = t2439 * t4625;
    let t51914 = F::new(0.5519e0) * t51913;
    let t51915 = t2439 * t4622;
    let t51917 = t698 * t15186;
    let t51919 = -F::new(0.3883875e1) * t51890 - F::new(0.1294625e1) * t51892 + F::cast_from(0.247573125e0_f64) * t51894 + F::new(0.82524375e-1) * t51896 - F::cast_from(0.485484375e1_f64) * t51899 + F::cast_from(0.6189328125e-1_f64) * t51902 - F::new(0.82785e-1) * t51907 - F::new(0.66228e0) * t51909 + F::new(0.11038e0) * t51911 + t51914 - F::cast_from(0.91983333333333333334e-1_f64) * t51915 - F::new(0.33114e0) * t51917;
    (t51907, t51909, t51911, t51913, t51915, t51917, t51919)
}
