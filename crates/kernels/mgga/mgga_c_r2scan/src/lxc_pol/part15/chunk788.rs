//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 788/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk788<F: Float>(t2049: F, t2312: F, t2317: F, t6804: F, t6806: F, t6809: F, t6868: F, t6874: F, t6876: F, t874: F, t875: F, t158: F) -> F {
    let t6879 = F::cast_from(0.1714584e0_f64) * t6804 - F::cast_from(0.1714584e0_f64) * t6806 * t2049 + F::cast_from(0.285764e-1_f64) * t6809 + F::cast_from(0.285764e-1_f64) * t6868 * t875 - F::cast_from(0.857292e-1_f64) * t2312 * t2317 * t874 + F::cast_from(0.571528e-1_f64) * t6874 * t6876;
    let t6880 = t6879 * t158;
    t6880
}
