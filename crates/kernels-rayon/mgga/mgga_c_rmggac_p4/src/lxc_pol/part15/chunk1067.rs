//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1067/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1067(t10018: f64, t7244: f64, t1916: f64, t2150: f64, t2868: f64, t41774: f64, t41790: f64, t41792: f64, t41812: f64, t41813: f64, t47478: f64, t47484: f64, t47487: f64, t47490: f64, t47493: f64, t47495: f64, t47500: f64, t47505: f64, t47510: f64, t8988: f64) -> f64 {
    let t47512 = t7244 * t10018;
    let t47515 = -0.19863479950205658386e-3_f64 * t41774 - 0.72732431077987577941e-1_f64 * t47478 - 0.11974241701863808564e0_f64 * t2868 * t8988 + t41790 + t41792 - 0.19957069503106347607e-1_f64 * t1916 * t2150 - 0.19863479950205658386e-4_f64 * t47484 + 0.8980681276397856423e-1_f64 * t47487 - 0.17961362552795712846e0_f64 * t47490 - 0.44903406381989282115e-1_f64 * t47493 + 0.31923449919973379548e-4_f64 * t47495 + 0.31923449919973379548e-4_f64 * t47500 + 0.31923449919973379548e-4_f64 * t47505 - 0.63846899839946759095e-4_f64 * t47510 + 0.99317399751028291929e-5_f64 * t47512 - t41812 + 0.59590439850616975157e-4_f64 * t41813;
    t47515
}
