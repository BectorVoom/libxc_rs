//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 944/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk944<F: Float>(t10805: F, t8862: F, t13346: F, t4349: F, t605: F, t11135: F, t10802: F, t27229: F, t11969: F, t1960: F, t977: F, t24215: F, t3553: F) -> (F, F, F, F, F, F) {
    let t46008 = F::new(4.0) * t8862 * t10805;
    let t46011 = F::new(12.0) * t4349 * t13346 * t605;
    let t46013 = F::new(4.0) * t8862 * t11135;
    let t46016 = F::new(12.0) * t27229 * t10802;
    let t46019 = F::new(2.0) * t1960 * t11969 * t977;
    let t46023 = F::new(2.0) * t24215 * t3553;
    (t46008, t46011, t46013, t46016, t46019, t46023)
}
