//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1168/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1168<F: Float>(t2409: F, t9716: F, t3959: F, t13989: F, t13999: F, t14002: F, t14114: F, t14742: F, t14745: F, t14749: F, t14752: F, t14755: F, t14759: F, t14768: F, t14770: F, t2408: F, t3066: F, t335: F) -> (F, F) {
    let t14772 = t2409 * t9716;
    let t14773 = t3959 * t14772;
    let t14775 = -t335 * t14742 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14745 + t3066 * t14749 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t14752 + t14755 / F::cast_from(1536.0_f64) + t13989 + t2408 * t14759 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t13999 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14002 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t14114 + t14768 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14770 - t14773 / F::cast_from(48.0_f64);
    (t14772, t14775)
}
