//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 943/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk943(t37481: f64, t40: f64, t1608: f64, t22817: f64, t52: f64, t5522: f64, t7837: f64, t409: f64, t5551: f64, t14: f64, t32213: f64, t1711: f64, t64: f64) -> (f64, f64, f64, f64, f64) {
    let t136595 = t40 * t37481;
    let t136597 = t1608 * t22817 * t136595;
    let t136604 = t7837 * t5522 * t52;
    let t136635 = t409 * t5551;
    let t136637 = t32213 * t14;
    let t136642 = t64 * t1711 * t5551;
    (t136597, t136604, t136635, t136637, t136642)
}
