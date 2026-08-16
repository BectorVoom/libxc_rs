//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1003/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1003(t1979: f64, t1982: f64, t458: f64, t8607: f64, t1163: f64, t2313: f64, t2189: f64, t3350: f64, t8515: f64, t8519: f64, t1971: f64, t236: f64, t5564: f64, t8517: f64) -> (f64, f64, f64, f64) {
    let t41993 = t8607 * t458 * t1979 * t1982;
    let t41999 = t2313 * t1163 * t1979 * t1982;
    let t42003 = t2189 * t8515 * t3350 * t8519;
    let t42007 = t8517 * t1971 * t236 * t5564;
    (t41993, t41999, t42003, t42007)
}
