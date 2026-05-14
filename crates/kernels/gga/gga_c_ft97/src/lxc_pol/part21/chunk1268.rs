//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1268/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1268<F: Float>(t23657: F, t23671: F, t30254: F, t379: F, t28: F, t30105: F, t586: F, t5890: F, t590: F, t558: F, t1369: F, t2112: F, t23609: F, t30187: F, t376: F, t30250: F) -> (F, F, F, F, F, F, F) {
    let t119682 = t23657 * t23671 * t30254 * t379;
    let t119687 = t5890 * t28 * t586 * t30105 * t590;
    let t119689 = t30105 * t558;
    let t119692 = t1369 * t28 * t2112 * t119689;
    let t119694 = t23609 * t376 * t30187;
    let t119695 = t119694 / 8.0;
    let t119697 = t5890 * t376 * t30250;
    (t119682, t119687, t119689, t119692, t119694, t119695, t119697)
}
