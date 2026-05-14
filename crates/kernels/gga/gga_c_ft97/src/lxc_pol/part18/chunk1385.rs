//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1385/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1385<F: Float>(t1882: F, t27281: F, t26978: F, t27191: F, t604: F, t27313: F, t27193: F, t6692: F, t8232: F, t2178: F, t6718: F, t1017: F, t104463: F, t104467: F, t13140: F, t144: F, t1901: F, t2190: F, t2210: F, t2230: F, t23548: F, t23938: F, t27016: F, t27414: F, t3281: F, t379: F, t446: F, t49414: F, t558: F, t569: F, t574: F, t605: F, t6615: F, t96035: F) -> (F,) {
    let t107379 = 2.0 / 27.0 * t1882 * t27281;
    let t107381 = 2.0 / 9.0 * t1882 * t26978;
    let t107399 = t604 * t27191;
    let t107412 = 2.0 / 9.0 * t1882 * t27313;
    let t107417 = 2.0 / 9.0 * t1882 * t27193;
    let t107418 = t8232 * t6692;
    let t107420 = t2178 * t6718;
    let t107425 = t107379 + t107381 - 4.0 / 3.0 * t1901 * t49414 * t27016 - 2.0 / 3.0 * t446 * t574 * t27414 * t558 - t446 * t574 * t2230 * t6615 / 3.0 - 4.0 / 9.0 * t96035 + t446 * t574 * t605 * t23938 * t1017 / 3.0 + 2.0 / 9.0 * t1901 * t2210 * t107399 * t379 - t446 * t144 * t104467 / 3.0 + 2.0 / 9.0 * t3281 * t569 * t605 * t23548 + t107412 + 2.0 / 3.0 * t446 * t144 * t104463 + t107417 - 4.0 / 27.0 * t107418 - 4.0 / 3.0 * t1901 * t13140 * t107420 * t2190;
    (t107425,)
}
