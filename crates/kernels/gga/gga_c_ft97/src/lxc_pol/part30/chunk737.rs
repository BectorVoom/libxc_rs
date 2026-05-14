//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 737/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk737<F: Float>(t34287: F, t7570: F, t1477: F, t684: F, t666: F, t461: F, t6343: F, t342: F, t630: F, t7574: F, t231: F, t6260: F, t1466: F, t1526: F, t2: F, t2320: F, t34284: F, t343: F, t6335: F, t6340: F, t7571: F) -> (F, F, F, F, F, F, F) {
    let t34289 = t7570 * t34287 / 18.0;
    let t34290 = t1477 * t684;
    let t34291 = t666 * t34290;
    let t34296 = t461 * t6343;
    let t34301 = t342 * t630 * t7574 / 12.0;
    let t34305 = t231 * t6260;
    let t34310 = (-t34284 * t7571 / 6.0 + t34289 + t1466 * t34291 / 18.0 + t1466 * t6340 / 3.0 - t7570 * t34296 / 6.0 - t34301 - t1526 * t2320 * t6335 / 12.0 - t342 * t343 * t34305 / 4.0) * t2;
    (t34289, t34290, t34291, t34296, t34301, t34305, t34310)
}
