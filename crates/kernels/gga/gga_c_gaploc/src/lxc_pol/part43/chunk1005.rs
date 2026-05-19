//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1005/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1005<F: Float>(t1456: F, t1457: F, t46941: F, t1445: F, t567: F, t40374: F, t40380: F, t40397: F, t40400: F, t47877: F, t587: F, t912: F) -> (F, F, F, F, F, F, F) {
    let t48066 = F::cast_from(0.35750489951850426669e0_f64) * t1456 * t1457 * t46941;
    let t48069 = F::cast_from(0.23005755572352449806e1_f64) * t567 * t1445 * t46941;
    let t48071 = F::cast_from(0.38342925953920749677e0_f64) * t40374;
    let t48073 = F::cast_from(0.51123901271894332903e0_f64) * t40380;
    let t48074 = F::cast_from(0.38342925953920749677e0_f64) * t40397;
    let t48076 = F::cast_from(0.76685851907841499354e0_f64) * t40400;
    let t48081 = t587 * t912 * t47877;
    (t48066, t48069, t48071, t48073, t48074, t48076, t48081)
}
