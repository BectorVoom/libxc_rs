//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1083/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1083(t41297: f64, t41308: f64, t41314: f64, t41319: f64, t41323: f64, t41294: f64, t41299: f64, t41302: f64, t41305: f64, t41311: f64, t41317: f64, t41321: f64, t41325: f64, t41327: f64, t41330: f64, t41332: f64) -> f64 {
    let t43588 = 0.24244143692662525982e0_f64 * t41297;
    let t43592 = 0.14546486215597515589e0_f64 * t41308;
    let t43594 = 0.14546486215597515589e0_f64 * t41314;
    let t43596 = 0.4838420607177634088e-2_f64 * t41319;
    let t43598 = 0.67737888500486877232e-2_f64 * t41323;
    let t43603 = 0.16934472125121719308e-2_f64 * t41294 - t43588 - 0.90317184667315836309e-2_f64 * t41299 - 0.72732431077987577945e-1_f64 * t41302 + 0.13637330827122670865e0_f64 * t41305 + t43592 - 0.5454932330849068346e-1_f64 * t41311 + t43594 - 0.2727466165424534173e0_f64 * t41317 - t43596 + 0.10160683275073031585e-1_f64 * t41321 + t43598 - 0.63504270469206447405e-2_f64 * t41325 + 0.67737888500486877232e-2_f64 * t41327 - 0.15241024912609547377e-1_f64 * t41330 + 0.5987120850931904282e-1_f64 * t41332;
    t43603
}
