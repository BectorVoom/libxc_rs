//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1086/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1086(t43515: f64, t43532: f64, t43550: f64, t43567: f64, t43586: f64, t43603: f64, t43619: f64, t43634: f64, t1614: f64, t2228: f64, t235: f64, t2868: f64, t36344: f64, t36379: f64, t36381: f64, t36383: f64, t37872: f64, t40621: f64, t40625: f64, t40627: f64, t40630: f64, t40637: f64, t40647: f64, t43492: f64, t504: f64, t515: f64, t8078: f64, t8273: f64, t884: f64, t9487: f64) -> (f64, f64, f64) {
    let t43637 = t43515 + t43532 + t43550 + t43567 + t43586 + t43603 + t43619 + t43634;
    let t43644 = t2228 * t1614;
    let t43652 = 0.638468998399467591e-4_f64 * t40621 - t37872 - t43492 - 0.39914139006212695214e-1_f64 * t504 * t9487 + 0.11974241701863808564e0_f64 * t40625 - 0.17961362552795712846e0_f64 * t40627 + 0.8980681276397856423e-1_f64 * t40630 - 0.11974241701863808564e0_f64 * t2868 * t8273 - 0.19957069503106347607e-1_f64 * t235 * t515 * t43637 + 0.10215503974391481456e-3_f64 * t40637 - 0.49658699875514145964e-4_f64 * t36344 - 0.11918087970123395032e-3_f64 * t36379 + 0.11974241701863808564e0_f64 * t884 * t43644 - 0.39726959900411316772e-4_f64 * t36381 - 0.39726959900411316772e-4_f64 * t36383 - 0.2727466165424534173e-1_f64 * t40647 - 0.59871208509319042821e-1_f64 * t2868 * t8078;
    (t43637, t43644, t43652)
}
