//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1299/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1299<F: Float>(t45972: F, t7565: F, t2121: F, t2247: F, t2251: F, t45963: F, t10309: F, t26754: F, t2123: F, t25120: F, t25159: F, t26783: F, t26786: F, t26789: F, t603: F, t606: F, t6960: F, t6963: F, t7566: F, t7576: F, t7579: F, t92662: F, t92672: F, t92674: F, t92692: F) -> F {
    let t96804 = t45972 * t7565;
    let t96810 = t2247 * t2251 * t2121;
    let t96824 = t45963 * t7565;
    let t96827 = t10309 * t26754;
    let t96830 = F::cast_from(35.0_f64) * t96804 * t92692 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7566 * t92662 - F::cast_from(5.0_f64) * t96810 * t6960 + t603 * t606 * t2121 * t92672 + t92674 * t2123 / F::cast_from(3.0_f64) + t25120 * t7576 + t25120 * t7579 + t6963 * t26783 + F::cast_from(2.0_f64) * t6963 * t26786 + t6963 * t26789 - F::cast_from(15.0_f64) * t96824 * t25159 - F::cast_from(15.0_f64) * t96827 * t25159;
    t96830
}
