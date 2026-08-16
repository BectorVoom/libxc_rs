//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1037/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1037(t2019: f64, t2020: f64, t8858: f64, t2010: f64, t2012: f64, t5757: f64, t4962: f64, t8854: f64, t5002: f64, t8850: f64, t1652: f64, t1971: f64, t495: f64, t515: f64, t7230: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41604 = t2019 * t2020 * t8858;
    let t41605 = 0.30487649791575028314e-3_f64 * t41604;
    let t41607 = t2010 * t2012 * t5757;
    let t41610 = t2010 * t2012 * t4962;
    let t41613 = t2019 * t2020 * t8854;
    let t41614 = 0.30487649791575028314e-3_f64 * t41613;
    let t41616 = t2010 * t2012 * t5002;
    let t41619 = t2019 * t2020 * t8850;
    let t41620 = 0.30487649791575028314e-3_f64 * t41619;
    let t41627 = t7230 * t1971 * t515 * t1652 * t495;
    (t41605, t41607, t41610, t41614, t41616, t41620, t41627)
}
