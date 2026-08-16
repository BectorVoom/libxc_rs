//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 550/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk550(t4623: f64, t4624: f64, t706: f64, t574: f64, t673: f64, t1648: f64, t682: f64, t1824: f64, t298: f64, t446: f64, t569: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4625 = t4623 * t4624;
    let t4626 = t706 * t4625;
    let t4629 = t673 * t574;
    let t4630 = t682 * t1648;
    let t4631 = t4630 * t1824;
    let t4632 = t4629 * t4631;
    let t4636 = t298 * t446 * t569;
    (t4625, t4626, t4629, t4630, t4631, t4632, t4636)
}
