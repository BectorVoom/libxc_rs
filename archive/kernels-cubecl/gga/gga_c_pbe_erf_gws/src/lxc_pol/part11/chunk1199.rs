//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1199/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1199<F: Float>(t10134: F, t12970: F, t12973: F, t12987: F, t138: F, t1577: F, t19407: F, t25918: F, t2902: F, t34210: F, t3675: F, t3683: F, t42742: F, t48733: F, t48752: F, t48774: F, t48807: F, t48823: F, t48829: F, t48843: F, t48856: F, t514: F, t5854: F, t8209: F, t985: F) -> F {
    let t48859 = (t48733 + t48752 + t48774 + t48807) * t138 - F::cast_from(4.0_f64) * t42742 * t985 + F::cast_from(12.0_f64) * t34210 * t3675 - F::cast_from(6.0_f64) * t10134 * t3683 - F::cast_from(24.0_f64) * t25918 * t12970 + F::cast_from(24.0_f64) * t8209 * t12973 - F::cast_from(4.0_f64) * t2902 * t12987 + F::cast_from(24.0_f64) * t19407 * t48823 - F::cast_from(36.0_f64) * t5854 * t3675 * t3683 + F::cast_from(6.0_f64) * t1577 * t48829 + F::cast_from(8.0_f64) * t1577 * t985 * t12987 - t514 * (t48843 + t48856);
    t48859
}
