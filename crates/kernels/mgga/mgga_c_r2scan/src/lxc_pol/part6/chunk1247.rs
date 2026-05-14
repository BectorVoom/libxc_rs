//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1247/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1247<F: Float>(t18789: F, t18791: F, t818: F, t8353: F, t19131: F, t6622: F, t990: F, t1216: F, t1248: F, t806: F, t2362: F, t409: F, t1000: F, t19107: F, t6636: F, t1256: F, t810: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23320 = 144.0 * t18789;
    let t23321 = 240.0 * t18791;
    let t23353 = t8353 * t818;
    let t23366 = t19131 * t990 * t6622;
    let t23376 = t1248 * t1216 * t806;
    let t23381 = t2362 * t409;
    let t23382 = 20.0 * t23381;
    let t23384 = t19107 * t1000 * t6636;
    let t23394 = t1256 * t1216 * t810;
    (t23320, t23321, t23353, t23366, t23376, t23381, t23382, t23384, t23394)
}
