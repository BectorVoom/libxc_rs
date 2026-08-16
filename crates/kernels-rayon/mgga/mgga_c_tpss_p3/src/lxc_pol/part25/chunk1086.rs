//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1086/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1086(t11568: f64, t14973: f64, t2741: f64, t242: f64, t2675: f64, t4989: f64, t946: f64, t140: f64, t4965: f64, t925: f64, t4984: f64, t8983: f64) -> (f64, f64, f64, f64) {
    let t14974 = t11568 * t14973;
    let t14975 = t2741 * t14974;
    let t14979 = t242 * t2675 * t4989;
    let t14980 = t946 * t14979;
    let t14986 = t140 * t4965;
    let t14987 = t925 * t14986;
    let t14991 = t8983 * t4984;
    (t14975, t14980, t14987, t14991)
}
