//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1023/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1023(t10800: f64, t2728: f64, t1960: f64, t3511: f64, t13166: f64, t1955: f64, t42501: f64, t42503: f64, t42506: f64, t42509: f64, t42512: f64, t42514: f64, t42516: f64, t42518: f64, t43410: f64, t43460: f64, t43524: f64, t43583: f64, t43637: f64, t43687: f64, t43747: f64, t43794: f64, t43859: f64, t43899: f64, t43948: f64, t44006: f64, t44061: f64, t44108: f64, t44153: f64, t44188: f64, t44194: f64, t44196: f64, t44198: f64, t44202: f64, t44203: f64, t44207: f64, t748: f64) -> f64 {
    let t44208 = t10800 * t2728;
    let t44211 = t1960 * t3511 * t2728;
    let t44213 = -t748 * (t43410 + t43460 + t43524 + t43583 + t43637 + t43687 + t43747 + t43794 + t43859 + t43899 + t43948 + t44006 + t44061 + t44108 + t44153 + t44188) - t42501 - t42503 - t44194 - t42506 - t1955 * t13166 + 4.0_f64 * t44196 - 2.0_f64 * t44198 + t44202 - 2.0_f64 * t44203 - t42509 - t42512 - t44207 - 2.0_f64 * t44208 + t42514 + t42516 + 4.0_f64 * t44211 - t42518;
    t44213
}
