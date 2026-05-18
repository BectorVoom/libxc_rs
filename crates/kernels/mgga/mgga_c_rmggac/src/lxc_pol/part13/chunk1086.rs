//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1086/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1086<F: Float>(t43515: F, t43532: F, t43550: F, t43567: F, t43586: F, t43603: F, t43619: F, t43634: F, t1614: F, t2228: F, t235: F, t2868: F, t36344: F, t36379: F, t36381: F, t36383: F, t37872: F, t40621: F, t40625: F, t40627: F, t40630: F, t40637: F, t40647: F, t43492: F, t504: F, t515: F, t8078: F, t8273: F, t884: F, t9487: F) -> (F, F, F) {
    let t43637 = t43515 + t43532 + t43550 + t43567 + t43586 + t43603 + t43619 + t43634;
    let t43644 = t2228 * t1614;
    let t43652 = F::new(0.638468998399467591e-4) * t40621 - t37872 - t43492 - F::new(0.39914139006212695214e-1) * t504 * t9487 + F::new(0.11974241701863808564e0) * t40625 - F::new(0.17961362552795712846e0) * t40627 + F::new(0.8980681276397856423e-1) * t40630 - F::new(0.11974241701863808564e0) * t2868 * t8273 - F::new(0.19957069503106347607e-1) * t235 * t515 * t43637 + F::new(0.10215503974391481456e-3) * t40637 - F::new(0.49658699875514145964e-4) * t36344 - F::new(0.11918087970123395032e-3) * t36379 + F::new(0.11974241701863808564e0) * t884 * t43644 - F::new(0.39726959900411316772e-4) * t36381 - F::new(0.39726959900411316772e-4) * t36383 - F::new(0.2727466165424534173e-1) * t40647 - F::new(0.59871208509319042821e-1) * t2868 * t8078;
    (t43637, t43644, t43652)
}
