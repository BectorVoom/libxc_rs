//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 898/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk898<F: Float>(t5478: F, t5482: F, t5437: F, t5443: F, t5449: F, t5452: F, t7775: F, t7779: F, t7780: F, t7781: F, t7784: F, t7788: F, t7790: F, t7792: F, t7795: F, t7797: F, t7799: F) -> (F, F, F) {
    let t7800 = F::new(8.0) / F::new(135.0) * t5478;
    let t7801 = F::new(8.0) / F::new(81.0) * t5482;
    let t7802 = -F::new(4.0) / F::new(27.0) * t5437 - t5443 + t5449 / F::new(3.0) + F::cast_from(0.60777777777777777777e-1_f64) * t5452 + t7775 + t7779 + t7780 - t7781 - t7784 - t7788 - t7790 - t7792 + t7795 + t7797 - t7799 + t7800 + t7801;
    (t7800, t7801, t7802)
}
