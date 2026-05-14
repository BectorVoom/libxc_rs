//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 774/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk774<F: Float>(t40564: F, t40567: F, t40570: F, t1456: F, t1457: F, t40546: F, t42086: F, t42421: F, t42422: F, t42425: F, t42429: F, t42432: F, t42435: F, t42438: F, t42442: F, t42444: F, t42448: F, t42452: F, t42455: F, t42456: F, t42457: F, t42458: F) -> (F,) {
    let t42459 = 0.1022478025437886658e1 * t40564;
    let t42460 = 0.25561950635947166451e1 * t40567;
    let t42461 = 0.29792074959875355558e-1 * t40570;
    let t42462 = -t42421 - 0.38342925953920749676e0 * t42422 - 0.38342925953920749676e0 * t42425 + t42429 - t42432 - 0.15889106645266856298e0 * t42435 + t42438 + t42442 - t42444 + 0.35750489951850426669e0 * t1456 * t1457 * t42086 - 0.14300195980740170668e1 * t42448 - 0.50050685932590597338e1 * t42452 + 0.38342925953920749676e0 * t40546 + t42455 - t42456 + t42457 - t42458 + t42459 - t42460 + t42461;
    (t42462,)
}
