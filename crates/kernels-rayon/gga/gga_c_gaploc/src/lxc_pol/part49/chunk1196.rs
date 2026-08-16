//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1196/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1196(t10532: f64, t10533: f64, t47803: f64, t1456: f64, t1457: f64, t46941: f64, t1445: f64, t567: f64, t40372: f64, t40374: f64, t42154: f64, t42157: f64, t42159: f64, t42161: f64, t42163: f64, t42166: f64, t42170: f64) -> f64 {
    let t48060 = t10532 * t10533 * t47803;
    let t48066 = 0.35750489951850426669e0_f64 * t1456 * t1457 * t46941;
    let t48069 = 0.23005755572352449806e1_f64 * t567 * t1445 * t46941;
    let t48070 = 0.63904876589867916128e-1_f64 * t40372;
    let t48071 = 0.38342925953920749677e0_f64 * t40374;
    let t48072 = t42154 + 0.27606906686822939767e2_f64 * t48060 + t42157 - t42159 - t42161 - 0.62115540045351614476e2_f64 * t42163 + 0.27606906686822939767e2_f64 * t42166 + t48066 + t48069 - t48070 - t48071 - t42170;
    t48072
}
