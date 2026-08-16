//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 921/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk921(t8726: f64, t8729: f64, t8732: f64, t8735: f64, t8739: f64, t8741: f64, t8746: f64, t8752: f64, t8756: f64, t8758: f64, t8761: f64, t8766: f64, t8771: f64) -> f64 {
    let t10589 = 0.60724609375000000008e-3_f64 * t8726 - 0.28985453471303521736e-5_f64 * t8729 - 0.28985453471303521736e-5_f64 * t8732 - 0.14492726735651760868e-5_f64 * t8735 + 0.28985453471303521736e-5_f64 * t8739 + 0.57970906942607043472e-5_f64 * t8741 + 0.51491428373437201895e-6_f64 * t8746 - 0.66398272271344937795e-7_f64 * t8752 + 0.1180561280984512994e-6_f64 * t8756 - 0.57970906942607043472e-5_f64 * t8758 + 0.59037814670138888894e-5_f64 * t8761 - 0.67530371184977617164e-6_f64 * t8766 - 0.15458908518028544927e-5_f64 * t8771;
    t10589
}
