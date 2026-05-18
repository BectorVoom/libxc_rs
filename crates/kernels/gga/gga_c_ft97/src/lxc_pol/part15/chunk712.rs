//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 712/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk712<F: Float>(t20448: F, t20460: F, t103: F, t82: F, t16541: F, t1901: F, t20307: F, t20397: F, t20401: F, t20405: F, t20409: F, t20413: F, t20417: F, t20421: F, t20424: F, t20428: F, t20431: F, t20435: F, t20439: F, t28: F, t446: F, t89: F) -> (F, F, F) {
    let t20461 = t20448 + t20460;
    let t20463 = t82 * t20461 * t103;
    let t20467 = F::new(2.0) / F::new(3.0) * t16541 - t446 * t20307 - t446 * t20397 / F::new(3.0) - t446 * t20401 - F::new(2.0) * t446 * t20405 + F::new(2.0) * t446 * t20409 - F::new(2.0) * t446 * t20413 + F::new(2.0) * t446 * t20417 - t446 * t20421 - t446 * t20424 / F::new(3.0) - t446 * t20428 - F::new(2.0) / F::new(3.0) * t1901 * t20431 + t1901 * t20435 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t1901 * t20439 + t89 * t28 * t20463 / F::new(3.0);
    (t20461, t20463, t20467)
}
