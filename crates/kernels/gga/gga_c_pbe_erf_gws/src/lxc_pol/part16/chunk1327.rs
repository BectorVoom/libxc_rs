//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1327/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1327<F: Float>(t53729: F, t51651: F, t51667: F, t51683: F, t51688: F, t52131: F, t52432: F, t53734: F, t53736: F, t53742: F, t53748: F, t53751: F, t53758: F, t53768: F, t53772: F, t8793: F) -> F {
    let t55351 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t53729;
    let t55367 = -t55351 + t53734 / F::cast_from(24.0_f64) - t53736 / F::cast_from(24.0_f64) + t53742 / F::cast_from(768.0_f64) - t53748 / F::cast_from(192.0_f64) - F::cast_from(35.0_f64) / F::cast_from(54.0_f64) * t51651 + t53751 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t51667 + t53758 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t52432 + t8793 * t52131 / F::cast_from(48.0_f64) - t53768 / F::cast_from(1536.0_f64) - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t51683 - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t51688 - t53772 / F::cast_from(48.0_f64);
    t55367
}
