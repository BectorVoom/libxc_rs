//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta439 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1853;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta439(t19021: f64, t964: f64, t973: f64, t981: f64, t3022: f64, t6227: f64, t11528: f64, t6110: f64, t2869: f64, t6142: f64, t11134: f64, t11560: f64, t15189: f64, t15483: f64, t15484: f64, t15485: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18944: f64, t18948: f64, t324: f64, t300: f64, t6184: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19023, t19025, t19027, t19029, t19031, t19045) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1853(t19021, t964, t973, t981, t3022, t6227, t11528, t6110, t2869, t6142, t11134, t11560, t15189, t15483, t15484, t15485, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
        let (t19046, t19048, t19049) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1854(t19045, t324, t300, t6184);
    (t19023, t19025, t19027, t19029, t19031, t19045, t19046, t19048, t19049)
}
