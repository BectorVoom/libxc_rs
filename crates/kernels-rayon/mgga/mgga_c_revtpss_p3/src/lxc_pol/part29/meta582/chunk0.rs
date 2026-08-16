//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1933/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1933(t11064: f64, t1113: f64, t27384: f64, t27799: f64, t98767: f64, t33: f64, t41154: f64, t98786: f64, t1711: f64, t2411: f64, t14365: f64, t1544: f64, t3351: f64) -> (f64, f64, f64, f64, f64) {
    let t100974 = t11064 * t1113;
    let t100975 = t100974 * t27384;
    let t100978 = t27799 * t98767;
    let t100981 = t41154 * t33;
    let t100982 = t100981 * t98786;
    let t100987 = t2411 * t1711;
    let t100988 = t100987 * t14365;
    let t100993 = t3351 * t1544;
    (t100975, t100978, t100982, t100988, t100993)
}
