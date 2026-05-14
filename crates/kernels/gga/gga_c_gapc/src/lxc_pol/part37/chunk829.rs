//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 829/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk829<F: Float>(t8668: F, t8671: F, t8678: F, t8682: F, t8688: F, t8691: F, t8694: F, t8698: F, t8702: F, t8705: F, t8707: F, t8717: F, t8720: F, t8726: F, t8729: F, t8732: F, t8735: F, t8739: F, t8741: F, t8746: F, t8752: F, t8756: F, t8758: F, t8761: F, t8766: F, t8771: F) -> (F, F) {
    let t10574 = -0.78385901460875530441e-2 * t8668 - 0.4048307291666666667e-4 * t8671 - 0.59037814670138888894e-5 * t8678 - 0.59037814670138888894e-5 * t8682 + 0.42233783114695867695e-6 * t8688 - 0.2318836277704281739e-4 * t8691 - 0.27801896084645508334e-2 * t8694 + 0.55603792169291016668e-2 * t8698 + 0.12974218172834570556e-1 * t8702 - 0.57970906942607043472e-5 * t8705 + 0.57970906942607043472e-5 * t8707 - 0.71809639497914566863e-8 * t8717 + 0.1349435763888888889e-4 * t8720;
    let t10589 = 0.60724609375000000008e-3 * t8726 - 0.28985453471303521736e-5 * t8729 - 0.28985453471303521736e-5 * t8732 - 0.14492726735651760868e-5 * t8735 + 0.28985453471303521736e-5 * t8739 + 0.57970906942607043472e-5 * t8741 + 0.51491428373437201895e-6 * t8746 - 0.66398272271344937795e-7 * t8752 + 0.1180561280984512994e-6 * t8756 - 0.57970906942607043472e-5 * t8758 + 0.59037814670138888894e-5 * t8761 - 0.67530371184977617164e-6 * t8766 - 0.15458908518028544927e-5 * t8771;
    (t10574, t10589)
}
