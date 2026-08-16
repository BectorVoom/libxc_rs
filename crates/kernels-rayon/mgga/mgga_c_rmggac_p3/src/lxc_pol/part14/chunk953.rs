//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 953/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk953(t1987: f64, t9087: f64, t2004: f64, t9090: f64, t2007: f64, t2604: f64, t35683: f64, t35691: f64, t40307: f64, t40314: f64, t40319: f64, t40324: f64, t40329: f64, t40332: f64, t40335: f64, t40337: f64, t40339: f64, t40343: f64, t40345: f64, t9025: f64) -> f64 {
    let t40347 = t9087 * t1987;
    let t40349 = t9090 * t2004;
    let t40350 = 0.19863479950205658386e-4_f64 * t40349;
    let t40351 = t9090 * t2007;
    let t40353 = 0.8980681276397856423e-1_f64 * t40307 - 0.11974241701863808564e0_f64 * t2604 * t9025 + 0.1064114997332445985e-4_f64 * t40314 - 0.31923449919973379548e-4_f64 * t40319 + 0.31923449919973379548e-4_f64 * t40324 + 0.31923449919973379548e-4_f64 * t40329 - 0.2927036860455597649e0_f64 * t40332 - 0.99317399751028291929e-5_f64 * t35683 + 0.17961362552795712846e0_f64 * t40335 + 0.5987120850931904282e-1_f64 * t40337 + 0.59590439850616975156e-4_f64 * t40339 + 0.20496175532535769484e-3_f64 * t35691 + 0.14905073231436680509e-2_f64 * t40343 + 0.12769379967989351819e-4_f64 * t40345 - 0.25538759935978703638e-4_f64 * t40347 + t40350 - 0.59590439850616975157e-4_f64 * t40351;
    t40353
}
