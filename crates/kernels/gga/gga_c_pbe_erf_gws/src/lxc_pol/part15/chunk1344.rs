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
    let t54660 = -F::new(35.0) / F::new(432.0) * t54621 - t54624 / F::new(48.0) - t54627 / F::new(48.0) - t54629 / F::new(24.0) - t51978 + t3066 * t2409 * t8734 * t14747 / F::new(24.0) - t54636 / F::new(96.0) + F::new(7.0) / F::new(144.0) * t51979 - F::new(35.0) / F::new(432.0) * t54639 + F::new(35.0) / F::new(432.0) * t54641 - t335 * t338 * t3200 * t4053 / F::new(48.0) - t54649 / F::new(768.0) - F::new(7.0) / F::new(288.0) * t51992 - t2408 * t9283 * t13888 * t9326 / F::new(24.0) + t9241 * t9283 * t1193 * t8939 / F::new(4.0);
    t54660
}
