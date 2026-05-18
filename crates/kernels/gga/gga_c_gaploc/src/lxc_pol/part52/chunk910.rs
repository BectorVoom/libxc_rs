//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 910/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk910<F: Float>(t46335: F, t1429: F, t2365: F, t35888: F, t35893: F, t4391: F, t41906: F, t11386: F, t9285: F, t10532: F, t10533: F, t46254: F) -> (F, F, F, F, F, F) {
    let t46336 = F::new(0.14896037479937677779e-1) * t46335;
    let t46338 = t1429 * t2365 * t35888;
    let t46339 = F::new(0.44688112439813033337e-1) * t46338;
    let t46341 = t4391 * t2365 * t35893;
    let t46342 = F::new(0.29792074959875355558e-1) * t46341;
    let t46343 = F::new(0.30674340763136599741e1) * t41906;
    let t46345 = F::new(0.35750489951850426669e0) * t9285 * t11386;
    let t46352 = F::new(0.27606906686822939767e2) * t10532 * t10533 * t46254;
    (t46336, t46339, t46342, t46343, t46345, t46352)
}
