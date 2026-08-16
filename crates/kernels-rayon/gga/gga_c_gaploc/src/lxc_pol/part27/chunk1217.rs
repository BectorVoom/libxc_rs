//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1217/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1217(t2508: f64, t2936: f64, t7124: f64, t21571: f64, t3420: f64, t10770: f64, t7129: f64, t10773: f64, t1024: f64, t7589: f64, t1843: f64, t21476: f64, t25289: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32548 = 0.23071578690426672851e-1_f64 * t2508 * t2936 * t7124;
    let t32553 = 0.76905262301422242837e-2_f64 * t21571 * t3420;
    let t32555 = 0.15381052460284448567e-1_f64 * t7129 * t10770;
    let t32557 = 0.15381052460284448567e-1_f64 * t7129 * t10773;
    let t32560 = 0.76905262301422242837e-2_f64 * t2508 * t7589 * t1024;
    let t32584 = t21476 * t1843 * t25289;
    (t32548, t32553, t32555, t32557, t32560, t32584)
}
