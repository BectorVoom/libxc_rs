//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 823/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk823(t14314: f64, t570: f64, t262: f64, t8620: f64, t1652: f64, t3080: f64, t41063: f64, t739: f64, t7577: f64, t7778: f64, t8946: f64, t903: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t74811 = t14314 * t570;
    let t74812 = t262 * t74811;
    let t74813 = t8620 * t74812;
    let t74815 = t3080 * t1652;
    let t74816 = t262 * t74815;
    let t74817 = t8620 * t74816;
    let t74824 = 0.5987120850931904282e-1_f64 * t739 * t7577 * t41063;
    let t74829 = t903 * t7778 * t8946;
    (t74811, t74812, t74813, t74815, t74816, t74817, t74824, t74829)
}
