//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 921/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk921<F: Float>(t8668: F, t8671: F, t8678: F, t8682: F, t8688: F, t8691: F, t8694: F, t8698: F, t8702: F, t8705: F, t8707: F, t8717: F, t8720: F) -> F {
    let t10574 = -F::new(0.78385901460875530441e-2) * t8668 - F::new(0.4048307291666666667e-4) * t8671 - F::new(0.59037814670138888894e-5) * t8678 - F::new(0.59037814670138888894e-5) * t8682 + F::new(0.42233783114695867695e-6) * t8688 - F::new(0.2318836277704281739e-4) * t8691 - F::new(0.27801896084645508334e-2) * t8694 + F::new(0.55603792169291016668e-2) * t8698 + F::new(0.12974218172834570556e-1) * t8702 - F::new(0.57970906942607043472e-5) * t8705 + F::new(0.57970906942607043472e-5) * t8707 - F::new(0.71809639497914566863e-8) * t8717 + F::new(0.1349435763888888889e-4) * t8720;
    t10574
}
