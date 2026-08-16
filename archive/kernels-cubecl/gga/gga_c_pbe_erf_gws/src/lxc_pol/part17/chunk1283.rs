//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1283/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1283<F: Float>(t14404: F, t19704: F, t51756: F, t51758: F, t51769: F, t51771: F, t51781: F, t51788: F, t53843: F, t53846: F, t53848: F, t53852: F, t53856: F, t53862: F, t53867: F, t53870: F) -> F {
    let t53872 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t51756 - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t51758 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t51769 - t53843 / F::cast_from(8.0_f64) - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t51771 + t53846 / F::cast_from(24.0_f64) + t53848 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t51781 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t51788 - F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t53852 + t53856 / F::cast_from(384.0_f64) + t19704 * t14404 / F::cast_from(48.0_f64) + t53862 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t53867 - t53870 / F::cast_from(1536.0_f64);
    t53872
}
