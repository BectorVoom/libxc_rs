//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1053/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1053(t44001: f64, t3005: f64, t9800: f64, t9829: f64, t13142: f64, t7416: f64, t2365: f64, t32215: f64, t6111: f64, t13019: f64, t4614: f64, t833: f64) -> (f64, f64, f64, f64, f64) {
    let t44002 = 0.15976219147466979032e-1_f64 * t44001;
    let t44004 = t9800 * t3005 * t9829;
    let t44005 = 0.19171462976960374838e1_f64 * t44004;
    let t44009 = t7416 * t13142;
    let t44010 = 0.15976219147466979032e-1_f64 * t44009;
    let t44012 = t6111 * t2365 * t32215;
    let t44018 = t833 * t4614 * t13019;
    (t44002, t44005, t44010, t44012, t44018)
}
