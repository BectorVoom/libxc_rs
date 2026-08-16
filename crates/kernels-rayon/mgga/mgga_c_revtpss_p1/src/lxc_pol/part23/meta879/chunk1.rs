//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2787/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2787(t1432: f64, t22307: f64, t686: f64, t72: f64, t1385: f64, t1437: f64, t2482: f64, t6843: f64, t4104: f64, t136: f64, t2457: f64, t3964: f64, t6888: f64) -> (f64, f64, f64, f64, f64) {
    let t74884 = t1432 * t22307 * t72 * t686;
    let t74886 = t1385 * t22307;
    let t74892 = t2482 * t1437 * t6843;
    let t74893 = t74892 * t4104;
    let t74901 = t3964 * t6888 * t136 * t2457;
    (t74884, t74886, t74892, t74893, t74901)
}
