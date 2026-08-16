//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1199/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1199(t10134: f64, t12970: f64, t12973: f64, t12987: f64, t138: f64, t1577: f64, t19407: f64, t25918: f64, t2902: f64, t34210: f64, t3675: f64, t3683: f64, t42742: f64, t48733: f64, t48752: f64, t48774: f64, t48807: f64, t48823: f64, t48829: f64, t48843: f64, t48856: f64, t514: f64, t5854: f64, t8209: f64, t985: f64) -> f64 {
    let t48859 = (t48733 + t48752 + t48774 + t48807) * t138 - 4.0_f64 * t42742 * t985 + 12.0_f64 * t34210 * t3675 - 6.0_f64 * t10134 * t3683 - 24.0_f64 * t25918 * t12970 + 24.0_f64 * t8209 * t12973 - 4.0_f64 * t2902 * t12987 + 24.0_f64 * t19407 * t48823 - 36.0_f64 * t5854 * t3675 * t3683 + 6.0_f64 * t1577 * t48829 + 8.0_f64 * t1577 * t985 * t12987 - t514 * (t48843 + t48856);
    t48859
}
