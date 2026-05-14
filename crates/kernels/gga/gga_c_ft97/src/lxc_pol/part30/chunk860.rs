//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 860/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk860<F: Float>(t25409: F, t7581: F, t143263: F, t143273: F, t143332: F, t143335: F, t143365: F, t34281: F, t6210: F, t34053: F, t870: F, t34074: F, t8392: F, t34078: F, t34070: F, t34204: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t143432 = t7581 * t25409;
    let t143497 = 8.0 / 9.0 * t143263;
    let t143500 = 10.0 / 9.0 * t143273;
    let t143518 = 4.0 / 9.0 * t143332;
    let t143519 = 4.0 / 9.0 * t143335;
    let t143528 = 2.0 / 9.0 * t143365;
    let t143538 = t6210 * t34281;
    let t143592 = t870 * t34053;
    let t143604 = t8392 * t34074;
    let t143606 = t8392 * t34078;
    let t143608 = t8392 * t34070;
    let t143610 = t8392 * t34204;
    (t143432, t143497, t143500, t143518, t143519, t143528, t143538, t143592, t143604, t143606, t143608, t143610)
}
