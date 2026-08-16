//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1240/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1240(t2508: f64, t2936: f64, t7124: f64, t21571: f64, t3420: f64, t10770: f64, t7129: f64, t10773: f64, t1024: f64, t7589: f64, t10784: f64, t1841: f64, t1881: f64, t2610: f64, t32214: f64, t32529: f64, t32532: f64, t32535: f64, t32539: f64, t32541: f64, t32543: f64, t32545: f64, t3464: f64, t5269: f64, t5396: f64) -> f64 {
    let t32548 = 0.23071578690426672851e-1_f64 * t2508 * t2936 * t7124;
    let t32553 = 0.76905262301422242837e-2_f64 * t21571 * t3420;
    let t32555 = 0.15381052460284448567e-1_f64 * t7129 * t10770;
    let t32557 = 0.15381052460284448567e-1_f64 * t7129 * t10773;
    let t32560 = 0.76905262301422242837e-2_f64 * t2508 * t7589 * t1024;
    let t32565 = -t32529 + t32532 + t32535 + 0.30762104920568897134e-1_f64 * t7129 * t10784 + t32539 + t32541 + t32543 + t32545 - t32548 + 0.15381052460284448567e-1_f64 * t5269 * t3464 * t1881 + t32553 + t32555 + t32557 + t32560 + 0.51270174867614828558e-2_f64 * t1841 * t5396 * t2610 * t32214;
    t32565
}
