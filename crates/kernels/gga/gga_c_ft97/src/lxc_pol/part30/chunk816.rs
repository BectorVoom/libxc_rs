//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 816/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk816<F: Float>(t1466: F, t34281: F, t6208: F, t7150: F, t1491: F, t1774: F, t7570: F, t1477: F, t684: F, t666: F, t461: F, t6343: F) -> (F, F, F, F, F, F, F) {
    let t34283 = t1466 * t34281 / F::new(9.0);
    let t34284 = t6208 * t7150;
    let t34287 = t1774 * t1491;
    let t34289 = t7570 * t34287 / F::new(18.0);
    let t34290 = t1477 * t684;
    let t34291 = t666 * t34290;
    let t34296 = t461 * t6343;
    (t34283, t34284, t34287, t34289, t34290, t34291, t34296)
}
