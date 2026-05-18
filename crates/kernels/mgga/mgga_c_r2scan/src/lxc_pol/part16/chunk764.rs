//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 764/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk764<F: Float>(t2049: F, t2312: F, t2317: F, t6804: F, t6806: F, t6809: F, t6868: F, t6874: F, t6876: F, t874: F, t875: F, t158: F) -> F {
    let t6879 = F::new(0.1714584e0) * t6804 - F::new(0.1714584e0) * t6806 * t2049 + F::new(0.285764e-1) * t6809 + F::new(0.285764e-1) * t6868 * t875 - F::new(0.857292e-1) * t2312 * t2317 * t874 + F::new(0.571528e-1) * t6874 * t6876;
    let t6880 = t6879 * t158;
    t6880
}
