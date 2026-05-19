//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1052/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1052<F: Float>(t190: F, t5261: F, t1045: F, t505: F, t13738: F, t8676: F, t21: F, t3142: F, t3712: F, t8654: F, t4043: F, t1030: F) -> (F, F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t26312 = t5261 * t190;
    let t26331 = t1045 * t505;
    let t26369 = t8676 * t13738;
    let t26396 = t3712 * t3142 * t21;
    let t26416 = t8654 * pi;
    let t26447 = t4043 * pi;
    let t26561 = t1030 * t26312;
    (t26312, t26331, t26369, t26396, t26416, t26447, t26561)
}
