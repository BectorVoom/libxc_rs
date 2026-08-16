//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2480;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2481;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta596(t19045: f64, t324: f64, t300: f64, t6184: f64, t983: f64, t15547: f64, t1642: f64, t4719: f64, t4725: f64, t6104: f64, t914: f64, t936: f64, t15416: f64, t1610: f64, t4590: f64, t4632: f64, t11134: f64, t11534: f64, t15127: f64, t15189: f64, t15503: f64, t15504: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18944: f64, t18948: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19046, t19048, t19049, t19051, t19053, t19055, t19056, t19058) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2480(t19045, t324, t300, t6184, t983, t15547, t1642, t4719, t4725, t6104, t914, t936);
        let (t19060, t19062, t19077) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2481(t15416, t1610, t4590, t4632, t11134, t11534, t15127, t15189, t15503, t15504, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
    (t19046, t19048, t19049, t19051, t19053, t19055, t19056, t19058, t19060, t19062, t19077)
}
