//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1453/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1453<F: Float>(t10043: F, t111221: F, t111223: F, t111224: F, t111472: F, t111507: F, t111509: F, t111512: F, t111515: F, t111518: F, t116014: F, t2359: F, t2776: F, t564: F, t6651: F, t7694: F, t8472: F, t9776: F) -> (F,) {
    let t123489 = -t111221 - t111223 - t111224 - t564 * t8472 * t9776 / 16.0 + t111472 + t116014 - t2776 * t2359 * t7694 / 8.0 - t564 * t6651 * t10043 / 8.0 - t111507 + t111509 + t111512 - t111515 + t111518;
    (t123489,)
}
