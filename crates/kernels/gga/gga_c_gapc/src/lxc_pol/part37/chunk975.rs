//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 975/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk975<F: Float>(t1038: F, t1648: F, t1839: F, t20198: F, t13790: F, t8676: F, t190: F, t5261: F, t1045: F, t505: F, t13738: F, t21: F, t3142: F, t3712: F, t8654: F, t4043: F) -> (F, F, F, F, F, F, F, F) {
    let t26102 = t1648 * t1839 * t1038 * t20198;
    let t26226 = t8676 * t13790;
    let t26312 = t5261 * t190;
    let t26331 = t1045 * t505;
    let t26369 = t8676 * t13738;
    let t26396 = t3712 * t3142 * t21;
    let t26416 = t8654 * M_PI;
    let t26447 = t4043 * M_PI;
    (t26102, t26226, t26312, t26331, t26369, t26396, t26416, t26447)
}
