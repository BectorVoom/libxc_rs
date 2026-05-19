//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1192/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1192<F: Float>(t40320: F, t13826: F, t1580: F, t46952: F, t568: F, t597: F, t600: F, t13821: F, t1628: F, t574: F, t13825: F, t13750: F, t1589: F, t557: F) -> (F, F, F, F, F, F) {
    let t48011 = F::cast_from(0.72851559312449424385e1_f64) * t40320;
    let t48013 = F::cast_from(0.23005755572352449806e1_f64) * t1580 * t13826;
    let t48017 = F::cast_from(0.23005755572352449806e1_f64) * t597 * t568 * t600 * t46952;
    let t48020 = F::cast_from(0.30674340763136599741e1_f64) * t574 * t1628 * t13821;
    let t48023 = F::cast_from(0.30674340763136599741e1_f64) * t597 * t1628 * t13825;
    let t48026 = F::cast_from(0.23833659967900284446e0_f64) * t557 * t1589 * t13750;
    (t48011, t48013, t48017, t48020, t48023, t48026)
}
