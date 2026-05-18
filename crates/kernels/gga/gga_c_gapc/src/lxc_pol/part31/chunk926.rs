//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 926/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk926<F: Float>(t8726: F, t8729: F, t8732: F, t8735: F, t8739: F, t8741: F, t8746: F, t8752: F, t8756: F, t8758: F, t8761: F, t8766: F, t8771: F) -> F {
    let t10589 = F::new(0.60724609375000000008e-3) * t8726 - F::new(0.28985453471303521736e-5) * t8729 - F::new(0.28985453471303521736e-5) * t8732 - F::new(0.14492726735651760868e-5) * t8735 + F::new(0.28985453471303521736e-5) * t8739 + F::new(0.57970906942607043472e-5) * t8741 + F::new(0.51491428373437201895e-6) * t8746 - F::new(0.66398272271344937795e-7) * t8752 + F::new(0.1180561280984512994e-6) * t8756 - F::new(0.57970906942607043472e-5) * t8758 + F::new(0.59037814670138888894e-5) * t8761 - F::new(0.67530371184977617164e-6) * t8766 - F::new(0.15458908518028544927e-5) * t8771;
    t10589
}
