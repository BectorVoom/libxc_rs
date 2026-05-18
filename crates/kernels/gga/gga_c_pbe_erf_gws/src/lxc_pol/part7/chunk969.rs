//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 969/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk969<F: Float>(t401: F, t5025: F, t2718: F, t658: F, t1251: F, t1721: F, t1715: F, t5065: F, t1697: F, t191: F, t205: F, t16974: F, t16979: F, t16995: F, t17005: F, t17022: F, t17038: F, t1714: F, t25: F, t5061: F, t657: F) -> F {
    let t17939 = t401 * t5025;
    let t17944 = t2718 * t658;
    let t17949 = t1251 * t1721;
    let t17951 = t1251 * t1715;
    let t17953 = t401 * t5065;
    let t17957 = t191 / t205 / t1697;
    let t17964 = -F::new(0.79999999999999999998e-1) * t25 * t1714 * t16995 - F::new(0.66666666666666666666e-2) * t25 * t1714 * t17005 - F::new(0.35555555555555555556e-1) * t17939 + F::new(0.35555555555555555554e-1) * t25 * t5061 * t17038 + F::new(0.79012345679012345678e-1) * t17944 - F::new(0.66666666666666666667e-2) * t25 * t657 * t16979 - F::new(0.44444444444444444445e-1) * t17949 - F::new(0.14814814814814814815e-1) * t17951 + F::new(0.79012345679012345679e-2) * t17953 - F::new(0.69135802469135802468e-2) * t25 * t17957 * t16974 - F::new(0.24e0) * t25 * t657 * t17022;
    t17964
}
