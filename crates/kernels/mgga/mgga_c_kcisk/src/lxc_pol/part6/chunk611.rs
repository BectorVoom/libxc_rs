//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 611/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk611<F: Float>(t9046: F, t9093: F, t1908: F, t5360: F, t6756: F, t8512: F, t8516: F, t8520: F, t2604: F, t1974: F, t5380: F, t5387: F, t6823: F, t8525: F, t8527: F, t8559: F, t8561: F, t8565: F, t8568: F, t8571: F) -> (F, F, F, F, F, F) {
    let t9094 = t9046 + t9093;
    let t9095 = t1908 * t9094;
    let t9103 = t5360 + 0.11415555555555555555e-1 * t6756 - 0.11415555555555555555e-1 * t8512 + 0.34246666666666666666e-1 * t8516 - 0.17123333333333333333e-1 * t8520;
    let t9108 = t2604 * t2604;
    let t9109 = t9108 * t1974;
    let t9124 = -0.17648625e1 * t8525 + 0.3529725e1 * t8527 + t5380 + 0.34431666666666666666e0 * t6756 - 0.34431666666666666667e0 * t8512 + 0.103295e1 * t8516 - 0.516475e0 * t8520 + 0.31558125e0 * t8559 + 0.6311625e0 * t8561 + t5387 + 0.13892666666666666667e0 * t6823 - 0.34731666666666666667e-1 * t8565 + 0.20839e0 * t8568 - 0.104195e0 * t8571;
    (t9094, t9095, t9103, t9108, t9109, t9124)
}
