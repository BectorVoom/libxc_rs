//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1149/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1149<F: Float>(t116316: F, t25878: F, t25928: F, t22993: F, t4606: F, t22958: F, t5674: F, t16060: F, t5691: F, t16065: F, t22952: F, t22953: F, t29716: F, t379: F, t116299: F, t116302: F, t116305: F, t116310: F, t116314: F, t92141: F, t92144: F) -> (F, F, F, F, F, F, F, F, F) {
    let t116318 = t25878 * t25928 * t116316;
    let t116320 = t22993 * t4606;
    let t116322 = t5674 * t22958 * t116320;
    let t116324 = t5691 * t16060;
    let t116326 = t5674 * t22958 * t116324;
    let t116328 = t5691 * t16065;
    let t116330 = t25878 * t22958 * t116328;
    let t116334 = t22952 * t22953 * t29716 * t379;
    let t116336 = -t116299 - 3.0 / 4.0 * t116302 + 4.0 * t116305 + t116310 / 6.0 + t92141 + t92144 + 5.0 / 27.0 * t116314 + 4.0 / 9.0 * t116318 - 2.0 / 3.0 * t116322 - 2.0 / 3.0 * t116326 - 4.0 / 3.0 * t116330 - t116334 / 6.0;
    (t116318, t116320, t116322, t116324, t116326, t116328, t116330, t116334, t116336)
}
