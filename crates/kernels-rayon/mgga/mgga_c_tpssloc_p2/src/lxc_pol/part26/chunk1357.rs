//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1357/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1357(t131: f64, t467: f64, t50: f64, t82510: f64, t10469: f64, t461: f64, t11715: f64, t11721: f64, t3032: f64, t3502: f64, t3508: f64, t1090: f64, t11498: f64, t11882: f64, t2148: f64, t24858: f64, t3248: f64, t3471: f64, t7283: f64, t7362: f64, t7381: f64, t85941: f64, t85943: f64, t85945: f64, t85947: f64, t85952: f64, t85955: f64) -> (f64, f64, f64) {
    let t85963 = t50 * t82510 * t131 * t467;
    let t85964 = t461 * t10469;
    let t85965 = t85964 * t11715;
    let t85966 = t3032 * t11721;
    let t85971 = t85964 * t3502;
    let t85972 = t3032 * t3508;
    let t85977 = -0.82246703342411321825e-2_f64 * t7283 * t11498 * t2148 - 0.24674011002723396548e-1_f64 * t7283 * t3471 * t7381 - 0.54831135561607547884e-2_f64 * t85941 - 0.27415567780803773942e-2_f64 * t85943 - 0.54831135561607547883e-2_f64 * t85945 - 0.82246703342411321826e-2_f64 * t7283 * t7362 * t85947 * t1090 + 0.18277045187202515961e-2_f64 * t85952 + 0.82246703342411321826e-2_f64 * t85955 - 0.16449340668482264365e-1_f64 * t7283 * t7362 * t24858 * t3248 + 0.49348022005446793095e-1_f64 * t85963 * t85965 * t11882 * t85966 - 0.49348022005446793095e-1_f64 * t85963 * t85971 * t11882 * t85972;
    (t85963, t85964, t85977)
}
