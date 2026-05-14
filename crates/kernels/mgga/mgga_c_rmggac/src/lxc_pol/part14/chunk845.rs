//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 845/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk845<F: Float>(t2004: F, t9090: F, t2007: F, t2604: F, t35683: F, t35691: F, t40307: F, t40314: F, t40319: F, t40324: F, t40329: F, t40332: F, t40335: F, t40337: F, t40339: F, t40343: F, t40345: F, t40347: F, t9025: F) -> (F,) {
    let t40349 = t9090 * t2004;
    let t40350 = 0.19863479950205658386e-4 * t40349;
    let t40351 = t9090 * t2007;
    let t40353 = 0.8980681276397856423e-1 * t40307 - 0.11974241701863808564e0 * t2604 * t9025 + 0.1064114997332445985e-4 * t40314 - 0.31923449919973379548e-4 * t40319 + 0.31923449919973379548e-4 * t40324 + 0.31923449919973379548e-4 * t40329 - 0.2927036860455597649e0 * t40332 - 0.99317399751028291929e-5 * t35683 + 0.17961362552795712846e0 * t40335 + 0.5987120850931904282e-1 * t40337 + 0.59590439850616975156e-4 * t40339 + 0.20496175532535769484e-3 * t35691 + 0.14905073231436680509e-2 * t40343 + 0.12769379967989351819e-4 * t40345 - 0.25538759935978703638e-4 * t40347 + t40350 - 0.59590439850616975157e-4 * t40351;
    (t40353,)
}
