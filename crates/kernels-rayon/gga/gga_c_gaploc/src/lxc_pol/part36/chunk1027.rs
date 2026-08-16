//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1027/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1027(t224: f64, t42919: f64, t43356: f64, t44213: f64, t44247: f64, t13247: f64, t42491: f64, t42494: f64, t42496: f64, t42499: f64, t42501: f64, t42503: f64, t42506: f64, t42509: f64, t42512: f64, t42514: f64, t42516: f64, t42518: f64, t42520: f64, t42523: f64, t42904: f64, t44244: f64, t44246: f64, t617: f64) -> f64 {
    let t44250 = t224 * (t42919 + t43356 + t44213 + t44247);
    let t44251 = t13247 * t617 + t42491 + t42494 + t42496 - t42499 + t42501 + t42503 + t42506 + t42509 + t42512 - t42514 - t42516 + t42518 - t42520 + t42523 + t42904 - t44244 - t44246 + t44250;
    t44251
}
