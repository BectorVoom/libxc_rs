//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 687/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk687<F: Float>(t1398: F, t745: F, t735: F, t1668: F, t591: F, t615: F, t1757: F, t1679: F, t584: F, t1685: F, t1684: F, t1763: F, t1942: F, t1762: F, t1835: F, t377: F) -> (F, F, F, F, F) {
    let t5938 = t1398 * t745;
    let t5940 = 0.21687162600603479684e-1 * t735 * t5938;
    let t5942 = t1668 * t591;
    let t5943 = t615 * t5942;
    let t5945 = 0.67745118933333333331e-2 * t1757 * t5943;
    let t5946 = t584 * t1679;
    let t5947 = t1685 * t591;
    let t5948 = t1684 * t5947;
    let t5950 = 0.254044196e-2 * t5946 * t5948;
    let t5957 = t1763 * t1942;
    let t5959 = 0.32530743900905219526e-1 * t1762 * t5957;
    let t5960 = t377 * t1835;
    (t5940, t5945, t5950, t5959, t5960)
}
