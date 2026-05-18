//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 589/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk589<F: Float>(t7703: F, t876: F, t1356: F, t2064: F, t321: F, t1550: F, t645: F, t839: F, t4044: F, t2057: F, t4601: F, t1173: F, t201: F) -> (F, F, F, F, F, F, F, F) {
    let t7704 = t7703 * t876;
    let t7705 = t1356 * t7704;
    let t7706 = F::new(0.11974241701863808564e0) * t7705;
    let t7707 = t2064 * t321;
    let t7708 = t1550 * t7707;
    let t7710 = t645 * t839;
    let t7711 = t4044 * t7710;
    let t7712 = F::new(0.17961362552795712846e0) * t7711;
    let t7713 = t4601 * t2057;
    let t7714 = F::new(0.8980681276397856423e-1) * t7713;
    let t7715 = t201 * t1173;
    (t7704, t7706, t7707, t7708, t7710, t7712, t7714, t7715)
}
