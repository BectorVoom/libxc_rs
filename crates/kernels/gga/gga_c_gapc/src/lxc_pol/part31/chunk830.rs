//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 830/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk830<F: Float>(t8557: F, t8560: F, t8564: F, t8568: F, t8572: F, t8575: F, t8579: F, t8581: F, t8583: F, t8586: F, t8591: F, t10433: F, t10445: F, t10458: F, t10470: F, t10484: F, t10496: F, t10509: F) -> (F,) {
    let t10521 = 0.86898242813537603826e-4 * t8557 + 0.43449121406768801913e-4 * t8560 + 0.2534532082061513445e-4 * t8564 - 0.86898242813537603826e-4 * t8568 + 0.2534532082061513445e-4 * t8572 - 0.24720812115595177536e-3 * t8575 - 0.86898242813537603826e-4 * t8579 + 0.5503555378190714909e-3 * t8581 + 0.17319302560753675207e-3 * t8583 - 0.20855578275249024918e-2 * t8586 + 0.41711156550498049836e-2 * t8591;
    let t10524 = t10433 + t10445 + t10458 + t10470 + t10484 + t10496 + t10509 + t10521;
    (t10524,)
}
