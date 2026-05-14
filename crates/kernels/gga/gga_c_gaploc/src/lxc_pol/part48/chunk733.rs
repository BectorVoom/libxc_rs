//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 733/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk733<F: Float>(t13327: F, t6313: F, t13277: F, t10223: F, t123: F, t13254: F, t13298: F, t13316: F, t1358: F, t2268: F, t2822: F, t3340: F, t42661: F, t42664: F, t42671: F, t44413: F, t44416: F, t44420: F, t44423: F, t44425: F, t44426: F, t44435: F, t44437: F, t488: F, t535: F, t6305: F, t999: F) -> (F,) {
    let t44439 = 0.37940008847568199465e-1 * t6313 * t13327;
    let t44443 = 0.22764005308540919679e0 * t6313 * t13277;
    let t44452 = -t44413 + t44416 - 0.47425011059460249332e-2 * t42661 + 0.47425011059460249332e-2 * t42664 - 0.142275033178380748e-1 * t42671 - t44420 - t44423 + t44425 - 0.31616674039640166221e-2 * t1358 * t44426 * t123 * t488 + 0.28455006635676149599e-1 * t2268 * t535 * t13298 - t44435 + t44437 + t44439 + 0.15176003539027279787e0 * t6313 * t13254 - t44443 + 0.56910013271352299198e-1 * t2268 * t2822 * t3340 + 0.56910013271352299198e-1 * t2268 * t999 * t10223 + 0.56910013271352299198e-1 * t6305 * t13316;
    (t44452,)
}
