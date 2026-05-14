//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1137/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1137<F: Float>(t22619: F, t29502: F, t415: F, t22563: F, t4466: F, t7983: F, t100556: F, t101498: F, t101505: F, t101532: F, t115432: F, t115603: F, t115608: F, t115617: F, t115654: F, t1669: F, t22591: F, t22755: F, t25679: F, t25792: F, t3019: F, t3099: F, t411: F, t4446: F, t5569: F, t73: F, t74254: F, t7889: F, t92358: F, t92440: F, t92441: F, t92456: F, t92689: F, t92809: F, t930: F, t93122: F, t93192: F) -> (F,) {
    let t116041 = t22619 * t415 * t29502;
    let t116055 = t7983 * t22563 * t4466;
    let t116077 = 0.23754828622903245155e-2 * t22619 * t411 * t29502 - 0.29693535778629056444e-3 * t116041 - 12.0 * t1669 * t92809 * t115603 + 8.0 * t1669 * t22755 * t115608 + 4.0 * t1669 * t22755 * t74254 + 0.46509801892875584e-1 * t92689 * t4446 + 0.13519760450715832853e-3 * t3019 * t116055 + 0.17816121467177433866e-3 * t93122 * t100556 * t115617 - 0.21120586720831816188e-4 * t92456 * t92358 * t930 * t25792 - 0.44540303667943584666e-4 * t5569 * t73 * t115432 - 0.88910709717637694816e-2 * t7889 * t22591 * t25679 * t3099 - 0.1134997482304526749e-1 * t93192 + t101498 + 0.85124811172839506172e-2 * t101505 - t101532 - 0.10338048737805743098e-3 * t92440 * t92441 * t115654;
    (t116077,)
}
