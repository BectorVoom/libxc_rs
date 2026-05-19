//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 840/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk840<F: Float>(t13307: F, t6305: F, t13262: F, t6313: F, t13327: F, t13277: F, t10223: F, t123: F, t13254: F, t13298: F, t13316: F, t1358: F, t2268: F, t2822: F, t3340: F, t42661: F, t42664: F, t42671: F, t44413: F, t44416: F, t44420: F, t44423: F, t44425: F, t44426: F, t488: F, t535: F, t999: F) -> F {
    let t44435 = F::cast_from(0.19918504644973304719e0_f64) * t6305 * t13307;
    let t44437 = F::cast_from(0.7588001769513639893e-1_f64) * t6313 * t13262;
    let t44439 = F::cast_from(0.37940008847568199465e-1_f64) * t6313 * t13327;
    let t44443 = F::cast_from(0.22764005308540919679e0_f64) * t6313 * t13277;
    let t44452 = -t44413 + t44416 - F::cast_from(0.47425011059460249332e-2_f64) * t42661 + F::cast_from(0.47425011059460249332e-2_f64) * t42664 - F::cast_from(0.142275033178380748e-1_f64) * t42671 - t44420 - t44423 + t44425 - F::cast_from(0.31616674039640166221e-2_f64) * t1358 * t44426 * t123 * t488 + F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t535 * t13298 - t44435 + t44437 + t44439 + F::cast_from(0.15176003539027279787e0_f64) * t6313 * t13254 - t44443 + F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t2822 * t3340 + F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t999 * t10223 + F::cast_from(0.56910013271352299198e-1_f64) * t6305 * t13316;
    t44452
}
