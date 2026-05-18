//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 619/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk619<F: Float>(t7954: F, t82: F, t110: F, t7955: F, t1651: F, t447: F, t499: F, t1901: F, t446: F, t8526: F, t8529: F, t8534: F, t8536: F, t8541: F, t8546: F, t8551: F, t8555: F, t8559: F, t8564: F, t8567: F, t8570: F, t8574: F) -> (F, F, F, F) {
    let t8577 = t7954 * t82;
    let t8579 = t8577 * t110 * t7955;
    let t8583 = t447 * t499 * t1651;
    let t8586 = t8526 / F::new(9.0) + F::new(2.0) * t446 * t8529 - t8534 + F::new(2.0) / F::new(3.0) * t1901 * t8536 - F::new(2.0) * t446 * t8541 + F::new(2.0) * t446 * t8546 + t446 * t8551 + t446 * t8555 - F::new(2.0) / F::new(3.0) * t1901 * t8559 - F::new(2.0) * t446 * t8564 + F::new(2.0) / F::new(9.0) * t8567 - F::new(2.0) / F::new(9.0) * t446 * t8570 - t446 * t8574 / F::new(9.0) - F::new(10.0) / F::new(81.0) * t446 * t8579 - t446 * t8583 / F::new(3.0);
    (t8577, t8579, t8583, t8586)
}
