//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1195/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1195(t54162: f64, t7970: f64, t7978: f64, t7968: f64, t27601: f64, t27607: f64, t18210: f64, t27616: f64, t12825: f64, t7980: f64, t27637: f64, t27559: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t94976 = t54162 * t7970;
    let t94977 = t7978 * t94976;
    let t94979 = t7968 * t94976;
    let t94981 = t27607 * t27601;
    let t94988 = t18210 * t27616;
    let t94989 = t7978 * t94988;
    let t94991 = t7968 * t94988;
    let t95001 = t7978 * t12825 * t7980;
    let t95004 = t7978 * t18210 * t27637;
    let t95006 = t18210 * t27559;
    (t94977, t94979, t94981, t94989, t94991, t95001, t95004, t95006)
}
