//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 959/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk959(t3961: f64, t4588: f64, t4582: f64, t1615: f64, t376: f64, t1022: f64, t3131: f64) -> (f64, f64, f64, f64) {
    let t4589 = t4588 * t3961;
    let t4590 = t4582 * t4589;
    let t4593 = t376 * t1615;
    let t4594 = t3131 * t1022;
    (t4589, t4590, t4593, t4594)
}
