//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1236/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1236<F: Float>(t113379: F, t1486: F, t3704: F, t6334: F, t852: F, t2399: F, t6308: F, t7063: F, t10248: F, t113062: F, t446: F, t24980: F, t2862: F, t4129: F, t6318: F, t856: F) -> (F, F, F, F, F, F) {
    let t113380 = 4.0 / 9.0 * t113379;
    let t113383 = t1486 * t3704 * t852 * t6334;
    let t113386 = t6308 * t2399 * t7063;
    let t113387 = t113386 / 9.0;
    let t113389 = t446 * t10248 * t113062;
    let t113394 = t24980 * t2862 * t6318 * t4129 * t856;
    (t113380, t113383, t113386, t113387, t113389, t113394)
}
