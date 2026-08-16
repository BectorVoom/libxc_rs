//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 758/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk758<F: Float>(t12894: F, t4540: F, t12762: F, t1457: F, t1572: F, t12766: F, t10122: F, t874: F, t1445: F, t574: F, t2877: F, t3149: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12896 = F::cast_from(0.21450293971110256001e1_f64) * t4540 * t12894;
    let t12897 = t1457 * t12762;
    let t12898 = t1572 * t12897;
    let t12900 = t1457 * t12766;
    let t12902 = F::cast_from(0.71500979903700853338e0_f64) * t1572 * t12900;
    let t12904 = t10122 * t874;
    let t12905 = t1445 * t12904;
    let t12906 = t574 * t12905;
    let t12909 = F::cast_from(0.35750489951850426669e0_f64) * t3149 * t2877;
    (t12896, t12897, t12898, t12900, t12902, t12904, t12905, t12906, t12909)
}
