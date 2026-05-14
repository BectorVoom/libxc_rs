//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1285/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1285<F: Float>(t31164: F, t8392: F, t111137: F, t11593: F, t124151: F, t124212: F, t13885: F, t18123: F, t18196: F, t18486: F, t18520: F, t18622: F, t18749: F, t1901: F, t24737: F, t24793: F, t2574: F, t2606: F, t265: F, t28140: F, t28299: F, t28300: F, t31029: F, t31182: F, t3281: F, t3746: F, t446: F, t5073: F, t5171: F, t53942: F, t6074: F, t6161: F, t6947: F, t724: F, t773: F, t97701: F, t97810: F) -> (F,) {
    let t124803 = t8392 * t31164;
    let t124832 = 4.0 / 9.0 * t3281 * t724 * t6947 * t3746 + 2.0 / 3.0 * t446 * t2574 * t773 * t31029 + 2.0 / 3.0 * t446 * t2574 * t265 * t124212 + 2.0 / 3.0 * t446 * t2574 * t265 * t124151 - t111137 + 2.0 / 9.0 * t1901 * t97701 * t5171 - 4.0 / 9.0 * t11593 * t24793 * t18520 - 2.0 / 27.0 * t124803 - 4.0 * t1901 * t28140 * t6074 * t18622 - 4.0 / 3.0 * t1901 * t53942 * t31182 - 4.0 / 3.0 * t1901 * t13885 * t97810 * t5073 - 4.0 / 3.0 * t1901 * t13885 * t24737 * t18196 - 2.0 / 3.0 * t1901 * t13885 * t24737 * t18486 - 4.0 * t1901 * t28299 * t28300 * t18749 + t1901 * t2606 * t6161 * t18123 / 9.0;
    (t124832,)
}
