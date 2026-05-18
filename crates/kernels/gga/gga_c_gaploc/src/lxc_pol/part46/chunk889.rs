//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 889/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk889<F: Float>(t2386: F, t3338: F, t544: F, t6514: F, t40549: F, t40555: F, t40558: F, t40561: F, t40564: F, t40567: F, t40570: F, t1456: F, t1457: F, t40546: F, t42086: F, t42421: F, t42422: F, t42425: F, t42429: F, t42432: F, t42435: F, t42438: F, t42442: F, t42444: F, t42448: F) -> F {
    let t42452 = t544 * t6514 * t3338 * t2386;
    let t42455 = F::new(0.11916829983950142223e0) * t40549;
    let t42456 = F::new(0.89376224879626066674e-1) * t40555;
    let t42457 = F::new(0.59584149919750711116e-1) * t40558;
    let t42458 = F::new(0.59584149919750711116e-1) * t40561;
    let t42459 = F::new(0.1022478025437886658e1) * t40564;
    let t42460 = F::new(0.25561950635947166451e1) * t40567;
    let t42461 = F::new(0.29792074959875355558e-1) * t40570;
    let t42462 = -t42421 - F::new(0.38342925953920749676e0) * t42422 - F::new(0.38342925953920749676e0) * t42425 + t42429 - t42432 - F::new(0.15889106645266856298e0) * t42435 + t42438 + t42442 - t42444 + F::new(0.35750489951850426669e0) * t1456 * t1457 * t42086 - F::new(0.14300195980740170668e1) * t42448 - F::new(0.50050685932590597338e1) * t42452 + F::new(0.38342925953920749676e0) * t40546 + t42455 - t42456 + t42457 - t42458 + t42459 - t42460 + t42461;
    t42462
}
