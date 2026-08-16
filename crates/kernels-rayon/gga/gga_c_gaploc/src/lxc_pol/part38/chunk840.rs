//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 840/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk840(t13307: f64, t6305: f64, t13262: f64, t6313: f64, t13327: f64, t13277: f64, t10223: f64, t123: f64, t13254: f64, t13298: f64, t13316: f64, t1358: f64, t2268: f64, t2822: f64, t3340: f64, t42661: f64, t42664: f64, t42671: f64, t44413: f64, t44416: f64, t44420: f64, t44423: f64, t44425: f64, t44426: f64, t488: f64, t535: f64, t999: f64) -> f64 {
    let t44435 = 0.19918504644973304719e0_f64 * t6305 * t13307;
    let t44437 = 0.7588001769513639893e-1_f64 * t6313 * t13262;
    let t44439 = 0.37940008847568199465e-1_f64 * t6313 * t13327;
    let t44443 = 0.22764005308540919679e0_f64 * t6313 * t13277;
    let t44452 = -t44413 + t44416 - 0.47425011059460249332e-2_f64 * t42661 + 0.47425011059460249332e-2_f64 * t42664 - 0.142275033178380748e-1_f64 * t42671 - t44420 - t44423 + t44425 - 0.31616674039640166221e-2_f64 * t1358 * t44426 * t123 * t488 + 0.28455006635676149599e-1_f64 * t2268 * t535 * t13298 - t44435 + t44437 + t44439 + 0.15176003539027279787e0_f64 * t6313 * t13254 - t44443 + 0.56910013271352299198e-1_f64 * t2268 * t2822 * t3340 + 0.56910013271352299198e-1_f64 * t2268 * t999 * t10223 + 0.56910013271352299198e-1_f64 * t6305 * t13316;
    t44452
}
