//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 412/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk412(t1979: f64, t8511: f64, t205: f64, t4443: f64, t671: f64, t3350: f64, t1462: f64, t236: f64, t1587: f64, t649: f64, t27: f64, t2084: f64, t570: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8512 = t8511 * t1979;
    let t8515 = t4443 * t205;
    let t8516 = t671 * t8515;
    let t8517 = t8516 * t3350;
    let t8518 = t236 * t1462;
    let t8525 = t649 * t1587;
    let t8526 = t27 * t8525;
    let t8532 = t2084 * t570;
    (t8512, t8516, t8517, t8518, t8526, t8532)
}
