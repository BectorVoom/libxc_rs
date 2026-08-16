//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2970/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2970(t4711: f64, t64504: f64, t981: f64, t23811: f64, t300: f64, t983: f64, t52238: f64, t78423: f64, t18898: f64, t52459: f64, t15258: f64, t19133: f64) -> (f64, f64, f64, f64, f64) {
    let t78703 = 0.51947577317044391277e2_f64 * t981 * t64504 * t4711;
    let t78704 = t300 * t23811;
    let t78706 = 0.5848223622634646207e0_f64 * t78704 * t983;
    let t78709 = 0.31168546390226634766e3_f64 * t52238 * t4711 * t78423;
    let t78712 = 0.30762056574649219974e4_f64 * t981 * t18898 * t52459;
    let t78715 = 0.31168546390226634765e3_f64 * t981 * t19133 * t15258;
    (t78703, t78706, t78709, t78712, t78715)
}
