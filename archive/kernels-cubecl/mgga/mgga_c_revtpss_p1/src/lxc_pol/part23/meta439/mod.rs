//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta439 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1853;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta439<F: Float>(t19021: F, t964: F, t973: F, t981: F, t3022: F, t6227: F, t11528: F, t6110: F, t2869: F, t6142: F, t11134: F, t11560: F, t15189: F, t15483: F, t15484: F, t15485: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F, t324: F, t300: F, t6184: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t19023, t19025, t19027, t19029, t19031, t19045) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1853::<F>(t19021, t964, t973, t981, t3022, t6227, t11528, t6110, t2869, t6142, t11134, t11560, t15189, t15483, t15484, t15485, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
        let (t19046, t19048, t19049) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1854::<F>(t19045, t324, t300, t6184);
    (t19023, t19025, t19027, t19029, t19031, t19045, t19046, t19048, t19049)
}
