//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1196/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1196<F: Float>(t10532: F, t10533: F, t47803: F, t1456: F, t1457: F, t46941: F, t1445: F, t567: F, t40372: F, t40374: F, t42154: F, t42157: F, t42159: F, t42161: F, t42163: F, t42166: F, t42170: F) -> F {
    let t48060 = t10532 * t10533 * t47803;
    let t48066 = F::cast_from(0.35750489951850426669e0_f64) * t1456 * t1457 * t46941;
    let t48069 = F::cast_from(0.23005755572352449806e1_f64) * t567 * t1445 * t46941;
    let t48070 = F::cast_from(0.63904876589867916128e-1_f64) * t40372;
    let t48071 = F::cast_from(0.38342925953920749677e0_f64) * t40374;
    let t48072 = t42154 + F::cast_from(0.27606906686822939767e2_f64) * t48060 + t42157 - t42159 - t42161 - F::cast_from(0.62115540045351614476e2_f64) * t42163 + F::cast_from(0.27606906686822939767e2_f64) * t42166 + t48066 + t48069 - t48070 - t48071 - t42170;
    t48072
}
