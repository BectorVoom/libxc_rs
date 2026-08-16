//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1234/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1234(t34607: f64, t5218: f64, t33273: f64, t5260: f64, t676: f64, t11543: f64, t8751: f64, t11425: f64, t3085: f64, t3664: f64, t8903: f64, t3691: f64, t8728: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34608 = t34607 * t5218;
    let t34611 = t5260 * t33273 * t676;
    let t34613 = t11543 * t8751;
    let t34615 = t11425 * t3085;
    let t34617 = t3664 * t8903;
    let t34619 = t3691 * t8728;
    (t34608, t34611, t34613, t34615, t34617, t34619)
}
