//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1262/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1262(t11627: f64, t12434: f64, t12560: f64, t12563: f64, t12565: f64, t12569: f64, t12573: f64, t12577: f64, t12706: f64, t41108: f64, t41109: f64, t41110: f64, t41111: f64, t41112: f64, t41113: f64, t42438: f64, t42815: f64, t42867: f64, t42924: f64, t42973: f64, t43761: f64, t43806: f64, t44001: f64, t8: f64) -> f64 {
    let t44006 = -t12434 + t41108 + t11627 - t41109 + t12706 + t8 * (t42438 + t42815 + t42867 + t42924 + t42973 + t43761 + t43806 + t44001) + t12560 - t41110 + t41111 + t41112 - t12563 + t12565 + t12569 + t12573 + t12577 + t41113;
    t44006
}
