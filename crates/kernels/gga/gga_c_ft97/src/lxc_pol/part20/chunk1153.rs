//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1153/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1153<F: Float>(t1449: F, t41408: F, t38953: F, t6849: F, t6837: F, t761: F, t12001: F, t28222: F, t107964: F, t107968: F, t11593: F, t13885: F, t13898: F, t14108: F, t14117: F, t14145: F, t14171: F, t14245: F, t1901: F, t242: F, t24737: F, t24793: F, t2579: F, t28299: F, t28300: F, t446: F, t53797: F, t97725: F, t97740: F, t97745: F, t97770: F, t98123: F) -> (F,) {
    let t110612 = t41408 * t1449;
    let t110626 = t38953 * t6849;
    let t110629 = t761 * t6837;
    let t110641 = t12001 * t28222;
    let t110653 = 8.0 * t1901 * t28299 * t110612 * t14145 - 4.0 * t1901 * t28299 * t28300 * t14108 - 4.0 / 3.0 * t1901 * t13885 * t24737 * t14245 + 2.0 / 3.0 * t97725 + 4.0 / 81.0 * t110626 + 8.0 / 81.0 * t97740 - 4.0 / 3.0 * t1901 * t13885 * t110629 * t2579 + 4.0 / 9.0 * t53797 * t98123 * t14171 - t97745 / 9.0 + 4.0 / 3.0 * t446 * t242 * t107964 - 22.0 / 27.0 * t110641 + 4.0 / 3.0 * t446 * t242 * t107968 - 4.0 / 9.0 * t11593 * t24793 * t13898 - 8.0 / 9.0 * t11593 * t24793 * t14117 + 8.0 / 27.0 * t97770;
    (t110653,)
}
