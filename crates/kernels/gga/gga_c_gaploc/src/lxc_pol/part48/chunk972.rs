//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 972/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk972<F: Float>(t1: F, t44426: F, t544: F, t10532: F, t10533: F, t46362: F, t37675: F, t901: F, t37573: F, t1415: F, t2413: F, t20796: F, t46094: F) -> (F, F, F, F, F) {
    let t46414 = t544 * t44426 * t1;
    let t46420 = F::cast_from(0.38649669361552115674e3_f64) * t10532 * t10533 * t46362;
    let t46421 = t37675 * t901;
    let t46422 = F::cast_from(0.14896037479937677779e-1_f64) * t46421;
    let t46423 = t37573 * t1;
    let t46424 = t1415 * t46423;
    let t46426 = F::cast_from(0.10725146985555128001e1_f64) * t46424 * t2413;
    let t46432 = F::cast_from(0.27606906686822939767e2_f64) * t20796 * t10533 * t46094;
    (t46414, t46420, t46422, t46426, t46432)
}
