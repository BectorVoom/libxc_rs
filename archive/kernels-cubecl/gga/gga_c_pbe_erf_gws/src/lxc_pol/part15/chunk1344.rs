//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1344/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1344<F: Float>(t1178: F, t371: F, t3983: F, t9258: F, t1193: F, t13888: F, t14747: F, t2408: F, t2409: F, t3066: F, t3200: F, t335: F, t338: F, t4053: F, t51978: F, t51979: F, t51992: F, t54621: F, t54624: F, t54627: F, t54629: F, t54636: F, t54639: F, t54641: F, t8734: F, t8939: F, t9241: F, t9283: F, t9326: F) -> F {
    let t54649 = t3983 * t371 * t1178 * t9258;
    let t54660 = -F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t54621 - t54624 / F::cast_from(48.0_f64) - t54627 / F::cast_from(48.0_f64) - t54629 / F::cast_from(24.0_f64) - t51978 + t3066 * t2409 * t8734 * t14747 / F::cast_from(24.0_f64) - t54636 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t51979 - F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t54639 + F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t54641 - t335 * t338 * t3200 * t4053 / F::cast_from(48.0_f64) - t54649 / F::cast_from(768.0_f64) - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t51992 - t2408 * t9283 * t13888 * t9326 / F::cast_from(24.0_f64) + t9241 * t9283 * t1193 * t8939 / F::cast_from(4.0_f64);
    t54660
}
