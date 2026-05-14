//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1153/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1153<F: Float>(t21139: F, t34286: F, t10513: F, t18067: F, t6964: F, t10524: F, t10527: F, t1397: F, t10314: F, t2476: F, t580: F, t2877: F, t30292: F, t2375: F, t26451: F, t26455: F) -> (F, F, F, F, F, F, F) {
    let t34288 = 0.50050685932590597338e1 * t34286 * t21139;
    let t34291 = 0.85801175884441024006e1 * t18067 * t6964 * t10513;
    let t34294 = 0.42900587942220512002e1 * t1397 * t10524 * t10527;
    let t34297 = 0.12269736305254639897e2 * t2476 * t580 * t10314;
    let t34299 = 0.35750489951850426669e0 * t30292 * t2877;
    let t34301 = 0.23833659967900284446e0 * t26451 * t2375;
    let t34303 = 0.23833659967900284446e0 * t26455 * t2375;
    (t34288, t34291, t34294, t34297, t34299, t34301, t34303)
}
