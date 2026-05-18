//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1223/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1223<F: Float>(t10691: F, t21665: F, t2932: F, t7064: F, t7177: F, t10698: F, t1841: F, t21476: F, t7313: F, t24321: F, t2558: F, t9647: F) -> (F, F, F, F, F) {
    let t32328 = t21665 * t10691;
    let t32329 = F::new(0.64087718584518535698e-3) * t32328;
    let t32331 = t7064 * t2932 * t7177;
    let t32332 = F::new(0.32043859292259267849e-3) * t32331;
    let t32333 = t1841 * t10698;
    let t32334 = F::new(0.25635087433807414279e-2) * t32333;
    let t32336 = t21476 * t2932 * t7313;
    let t32337 = F::new(0.64087718584518535698e-3) * t32336;
    let t32339 = t9647 * t24321 * t2558;
    (t32329, t32332, t32334, t32337, t32339)
}
