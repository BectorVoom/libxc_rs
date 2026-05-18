//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1044/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1044<F: Float>(t1154: F, t6455: F, t254: F, t9404: F, t906: F, t3261: F, t6416: F, t2074: F, t274: F, t1123: F, t2255: F, t2338: F, t3252: F) -> (F, F, F, F, F) {
    let t9457 = t6455 * t1154;
    let t9459 = t254 * t9404;
    let t9460 = t9459 * t906;
    let t9464 = F::new(7.0) / F::new(576.0) * t6416 * t3261;
    let t9465 = t274 * t2074;
    let t9467 = t2255 * t1123 * t9465;
    let t9470 = t3252 * t2338;
    (t9457, t9460, t9464, t9467, t9470)
}
