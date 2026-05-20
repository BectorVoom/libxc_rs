//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3240/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3240<F: Float>(t10301: F, t10309: F, t1497: F, t21809: F, t2242: F, t2247: F, t22656: F, t22659: F, t22742: F, t4173: F, t4241: F, t45963: F, t45972: F, t5816: F, t5872: F, t603: F, t644: F, t85141: F, t85177: F, t85206: F, t85300: F) -> F {
    let t85305 = -F::new(12.0) * t4173 * t21809 - F::new(120.0) * t45963 * t22656 + F::new(840.0) * t45972 * t22656 * t644 - F::new(360.0) * t10309 * t5816 * t4241 + F::new(60.0) * t10301 * t22659 - F::new(360.0) * t10309 * t22659 * t644 + F::new(60.0) * t2247 * t4241 * t5872 + F::new(60.0) * t2247 * t1497 * t21809 - F::new(4.0) * t2242 * t22742 + F::new(20.0) * t2247 * t22742 * t644 - F::new(4.0) * t603 * (t85141 + t85177 + t85206 + t85300);
    t85305
}
