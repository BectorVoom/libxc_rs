//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 887/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk887<F: Float>(t10362: F, t289: F, t287: F, t41512: F, t2360: F, t274: F, t123: F, t41670: F, t805: F, t2347: F, t197: F, t8991: F, t9606: F) -> (F, F, F, F, F, F) {
    let t43585 = F::cast_from(1.0_f64) / t10362 / t289;
    let t43586 = t287 * t43585;
    let t43631 = F::cast_from(0.4939111192043895748e-1_f64) * t41512;
    let t43691 = t274 * t2360;
    let t43712 = t123 / t805 / t41670;
    let t43731 = t274 * t2347;
    let t43742 = t8991 / t197 / t9606;
    (t43586, t43631, t43691, t43712, t43731, t43742)
}
