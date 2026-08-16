//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1146/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1146(t291: f64, t4951: f64, t13511: f64, t14538: f64, t14543: f64, t14548: f64, t14551: f64, t2872: f64, t4963: f64, t9883: f64, t9906: f64, t991: f64, t9910: f64, t9918: f64, t9940: f64, t9957: f64, t9961: f64, t9970: f64) -> f64 {
    let t14554 = t4951 * t291;
    let t14555 = t14554 * t13511;
    let t14561 = t9883 - t9906 / 162.0_f64 - t9910 / 432.0_f64 - t9918 / 648.0_f64 - t9940 / 432.0_f64 - t14538 + t2872 * t4963 / 54.0_f64 + t991 * t14543 / 144.0_f64 - t991 * t14548 / 72.0_f64 - t991 * t14551 / 144.0_f64 - t991 * t14555 / 36.0_f64 + t9957 / 864.0_f64 + t9961 / 648.0_f64 + t9970 / 81.0_f64;
    t14561
}
