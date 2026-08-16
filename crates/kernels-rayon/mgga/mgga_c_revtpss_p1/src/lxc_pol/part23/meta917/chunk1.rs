//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2958/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2958(t23696: f64, t3022: f64, t15537: f64, t6206: f64, t981: f64, t19049: f64, t4725: f64, t23451: f64, t41235: f64, t41238: f64, t972: f64, t23446: f64) -> (f64, f64, f64, f64, f64) {
    let t78446 = 0.5848223622634646207e0_f64 * t3022 * t23696;
    let t78449 = 0.35089341735807877242e1_f64 * t981 * t15537 * t6206;
    let t78451 = 0.35089341735807877242e1_f64 * t19049 * t4725;
    let t78456 = 0.91082604192152556044e5_f64 * t981 * t41235 * t23451 * t41238 * t972;
    let t78458 = 0.35089341735807877242e1_f64 * t3022 * t23446;
    (t78446, t78449, t78451, t78456, t78458)
}
