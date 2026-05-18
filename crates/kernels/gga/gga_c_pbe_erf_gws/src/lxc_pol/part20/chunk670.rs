//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 670/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk670<F: Float>(t3258: F, t3757: F, t2255: F, t1133: F, t274: F, t343: F, t1123: F, t3123: F, t3134: F, t1220: F, t1278: F, t1288: F, t1296: F, t1328: F, t1330: F, t1335: F, t1338: F, t1440: F, t1450: F, t3341: F, t3362: F, t3702: F) -> (F, F, F, F, F, F) {
    let t3758 = t3258 * t3757;
    let t3759 = t2255 * t3758;
    let t3762 = t274 * t1133;
    let t3763 = t3762 * t343;
    let t3764 = t1123 * t3763;
    let t3765 = t2255 * t3764;
    let t3769 = t3123 * t3134 / F::new(48.0);
    let t3770 = t1220 + t1328 - t1330 + t1335 + t1338 + t1450 - t1278 + t1288 + t1296 + t1440 - t3341 + t3362 - t3702;
    (t3759, t3763, t3764, t3765, t3769, t3770)
}
