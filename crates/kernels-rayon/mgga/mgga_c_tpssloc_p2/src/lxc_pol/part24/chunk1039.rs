//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1039/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1039(t11712: f64, t11880: f64, t11720: f64, t491: f64, t11721: f64, t6739: f64, t10471: f64, t3502: f64, t3508: f64, t11624: f64, t3612: f64, t1215: f64, t3590: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11881 = t11712 * t11880;
    let t11882 = t491 * t11720;
    let t11883 = t6739 * t11721;
    let t11884 = t11882 * t11883;
    let t11887 = t10471 * t3502;
    let t11888 = t11712 * t11887;
    let t11889 = t6739 * t3508;
    let t11890 = t11882 * t11889;
    let t11893 = t11624 * t3612;
    let t11896 = t3590 * t1215;
    (t11881, t11882, t11884, t11888, t11890, t11893, t11896)
}
