//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1355/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1355(t225: f64, t85754: f64, t3545: f64, t7372: f64, t7378: f64, t24698: f64, t7327: f64, t11148: f64, t11504: f64, t1186: f64, t11888: f64, t11889: f64, t15022: f64, t2148: f64, t24589: f64, t24784: f64, t24788: f64, t24806: f64, t24829: f64, t24833: f64, t24859: f64, t3477: f64, t3624: f64, t7283: f64, t7363: f64, t7373: f64, t7377: f64, t7381: f64, t7386: f64, t85836: f64) -> f64 {
    let t85909 = t85754 * t225;
    let t85917 = t7372 * t3545;
    let t85918 = t85917 * t7378;
    let t85920 = t24698 * t7327;
    let t85933 = -3.0_f64 * t3624 * t7386 * t15022 - 6.0_f64 * t11888 * t85836 * t11889 - 0.24674011002723396548e-1_f64 * t7283 * t3477 * t7381 - 0.82246703342411321825e-2_f64 * t7283 * t11504 * t2148 - 0.8529287754027840782e-2_f64 * t7283 * t85909 * t7363 * t11148 + 0.16449340668482264365e-1_f64 * t24589 * t24788 * t24859 - 0.54831135561607547884e-2_f64 * t85918 - 0.24674011002723396548e-1_f64 * t7373 * t85920 * t7377 - 0.49348022005446793095e-1_f64 * t7373 * t24833 * t24784 - 0.24674011002723396548e-1_f64 * t7373 * t24833 * t24806 - 0.24674011002723396548e-1_f64 * t7283 * t1186 * t24829;
    t85933
}
