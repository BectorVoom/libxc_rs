//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1217/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1217<F: Float>(t2508: F, t2936: F, t7124: F, t21571: F, t3420: F, t10770: F, t7129: F, t10773: F, t1024: F, t7589: F, t1843: F, t21476: F, t25289: F) -> (F, F, F, F, F, F) {
    let t32548 = F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t2936 * t7124;
    let t32553 = F::cast_from(0.76905262301422242837e-2_f64) * t21571 * t3420;
    let t32555 = F::cast_from(0.15381052460284448567e-1_f64) * t7129 * t10770;
    let t32557 = F::cast_from(0.15381052460284448567e-1_f64) * t7129 * t10773;
    let t32560 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t7589 * t1024;
    let t32584 = t21476 * t1843 * t25289;
    (t32548, t32553, t32555, t32557, t32560, t32584)
}
