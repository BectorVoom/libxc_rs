//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 982/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk982<F: Float>(t1901: F, t27203: F, t27205: F, t27226: F, t27276: F, t30390: F, t30394: F, t30397: F, t30401: F, t30405: F, t30409: F, t30413: F, t30417: F, t30420: F, t30424: F, t30428: F, t30432: F, t30436: F, t446: F) -> (F,) {
    let t30443 = 2.0 / 3.0 * t446 * t30390 - 4.0 / 3.0 * t1901 * t30394 + 2.0 / 9.0 * t1901 * t30397 + 2.0 / 9.0 * t1901 * t30401 + t1901 * t30405 / 9.0 + 2.0 / 27.0 * t1901 * t30409 - 2.0 / 9.0 * t1901 * t30413 - 4.0 / 3.0 * t1901 * t30417 + 2.0 / 9.0 * t1901 * t30420 + 2.0 / 9.0 * t1901 * t30424 + t1901 * t30428 / 9.0 + 2.0 / 27.0 * t1901 * t30432 + 2.0 / 3.0 * t446 * t30436 + 2.0 / 9.0 * t27203 + 2.0 / 9.0 * t27205 + 2.0 / 9.0 * t27226 + 2.0 / 27.0 * t27276;
    (t30443,)
}
