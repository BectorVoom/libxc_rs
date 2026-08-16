//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1003/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1003(t11338: f64, t3579: f64, t3465: f64, t797: f64, t495: f64, t1146: f64, t2881: f64, t3718: f64, t498: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12049 = t3579 * t11338;
    let t12050 = t12049 / 4.0_f64;
    let t12051 = t3465 * t797;
    let t12052 = t495 * t12051;
    let t12053 = t3579 * t12052;
    let t12054 = t12053 / 4.0_f64;
    let t12055 = t1146 * t2881;
    let t12056 = t498 * t3718;
    (t12049, t12050, t12051, t12052, t12053, t12054, t12055, t12056)
}
