//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 933/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk933<F: Float>(t12054: F, t8833: F, t13565: F, t6484: F, t13334: F, t6: F, t5: F, t13571: F, t6203: F, t37997: F, t9016: F, t3128: F, t38870: F, t2083: F, t3373: F, t1114: F, t346: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44672 = t12054 * t8833;
    let t44695 = t6484 * t13565;
    let t44710 = t6 * t13334;
    let t44741 = t5 * t13334;
    let t44763 = t6203 * t13571;
    let t44814 = t9016 * t37997;
    let t44889 = t3128 * t38870;
    let t44900 = t2083 * t3373;
    let t44902 = t1114 * t44900 * t346;
    (t44672, t44695, t44710, t44741, t44763, t44814, t44889, t44900, t44902)
}
