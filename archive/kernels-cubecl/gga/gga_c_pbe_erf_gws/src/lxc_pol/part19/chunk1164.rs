//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1164/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1164<F: Float>(t14716: F, t4213: F, t840: F, t14745: F, t4230: F, t14752: F, t13974: F, t14333: F, t14722: F, t14727: F, t14729: F, t14731: F, t14734: F, t14737: F) -> (F, F, F) {
    let t14986 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t14716;
    let t14989 = t840 * t4213;
    let t14996 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t14745;
    let t14997 = t840 * t4230;
    let t14999 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14752;
    let t15000 = -t14986 - t14722 / F::cast_from(1536.0_f64) + t14727 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t14989 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t14333 + t14729 / F::cast_from(24.0_f64) + t14731 / F::cast_from(8.0_f64) - t14734 / F::cast_from(48.0_f64) + t13974 + t14737 / F::cast_from(48.0_f64) + t14996 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t14997 + t14999;
    (t14989, t14997, t15000)
}
