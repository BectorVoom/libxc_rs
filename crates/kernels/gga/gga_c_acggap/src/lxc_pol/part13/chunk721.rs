//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 721/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk721<F: Float>(t1410: F, t157: F, t609: F, t2152: F, t2122: F, t556: F, t2147: F, t463: F, t525: F, t7932: F, t2146: F, t7307: F, t7889: F, t7931: F, t8393: F, t8398: F, t8400: F, t8403: F, t8408: F, t8411: F, t8415: F, t8420: F, t8424: F, t8428: F) -> (F, F, F, F, F, F) {
    let t8432 = t609 * t1410 * t157;
    let t8433 = t2152 * t8432;
    let t8436 = t2122 * t556;
    let t8437 = t2147 * t8436;
    let t8440 = t525 * t463;
    let t8441 = t7932 * t8440;
    let t8444 = 0.17347256376410398924e1 * t7307 + 0.8673628188205199462e0 * t2146 * t8393 - 0.8673628188205199462e0 * t8398 + 0.4336814094102599731e0 * t8400 * t8403 - 0.8673628188205199462e0 * t8408 - t7889 - 0.26020884564615598386e1 * t2146 * t8411 + 0.8673628188205199462e0 * t2146 * t8415 + 0.8673628188205199462e0 * t8420 - 0.8673628188205199462e0 * t8424 + 0.4336814094102599731e0 * t2146 * t8428 + 0.4336814094102599731e0 * t2146 * t8433 + 0.8673628188205199462e0 * t2146 * t8437 - 0.8673628188205199462e0 * t7931 * t8441;
    (t8433, t8436, t8437, t8440, t8441, t8444)
}
