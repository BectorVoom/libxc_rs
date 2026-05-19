//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 875/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk875<F: Float>(t8726: F, t8729: F, t8732: F, t8735: F, t8739: F, t8741: F, t8746: F, t8752: F, t8756: F, t8758: F, t8761: F, t8766: F, t8771: F) -> F {
    let t10589 = F::cast_from(0.60724609375000000008e-3_f64) * t8726 - F::cast_from(0.28985453471303521736e-5_f64) * t8729 - F::cast_from(0.28985453471303521736e-5_f64) * t8732 - F::cast_from(0.14492726735651760868e-5_f64) * t8735 + F::cast_from(0.28985453471303521736e-5_f64) * t8739 + F::cast_from(0.57970906942607043472e-5_f64) * t8741 + F::cast_from(0.51491428373437201895e-6_f64) * t8746 - F::cast_from(0.66398272271344937795e-7_f64) * t8752 + F::cast_from(0.1180561280984512994e-6_f64) * t8756 - F::cast_from(0.57970906942607043472e-5_f64) * t8758 + F::cast_from(0.59037814670138888894e-5_f64) * t8761 - F::cast_from(0.67530371184977617164e-6_f64) * t8766 - F::cast_from(0.15458908518028544927e-5_f64) * t8771;
    t10589
}
