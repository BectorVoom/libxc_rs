//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1040/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1040<F: Float>(t38643: F, t38645: F, t38647: F, t38675: F, t34713: F, t34717: F, t37200: F, t37201: F, t37202: F, t37203: F, t38653: F, t38658: F, t38663: F, t38678: F, t38680: F, t38685: F, t38695: F) -> F {
    let t42696 = F::new(0.11918087970123395032e-3) * t38643;
    let t42697 = F::new(0.11918087970123395032e-3) * t38645;
    let t42698 = F::new(0.39726959900411316772e-4) * t38647;
    let t42702 = F::new(0.15965655602485078085e0) * t38675;
    let t42709 = -t42696 + t42697 + t42698 - t37200 + F::new(0.5107751987195740728e-4) * t38653 - F::new(0.5107751987195740728e-4) * t38658 - F::new(0.1702583995731913576e-4) * t38663 - t42702 + F::new(0.5987120850931904282e-1) * t38678 + F::new(0.11974241701863808564e0) * t38680 + F::new(0.11974241701863808564e0) * t38685 - F::new(0.17961362552795712846e0) * t38695 + t37201 + t37202 - t37203 - F::new(0.10248087766267884741e-3) * t34713 + F::new(0.1440846329149835838e-2) * t34717;
    t42709
}
