//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 657/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk657<F: Float>(t16541: F, t1901: F, t20307: F, t20397: F, t20401: F, t20405: F, t20409: F, t20413: F, t20417: F, t20421: F, t20424: F, t20428: F, t20431: F, t20435: F, t20439: F, t20463: F, t28: F, t446: F, t89: F) -> (F,) {
    let t20467 = 2.0 / 3.0 * t16541 - t446 * t20307 - t446 * t20397 / 3.0 - t446 * t20401 - 2.0 * t446 * t20405 + 2.0 * t446 * t20409 - 2.0 * t446 * t20413 + 2.0 * t446 * t20417 - t446 * t20421 - t446 * t20424 / 3.0 - t446 * t20428 - 2.0 / 3.0 * t1901 * t20431 + t1901 * t20435 / 3.0 - 2.0 / 3.0 * t1901 * t20439 + t89 * t28 * t20463 / 3.0;
    (t20467,)
}
