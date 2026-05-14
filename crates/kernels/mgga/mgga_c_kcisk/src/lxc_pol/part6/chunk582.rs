//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 582/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk582<F: Float>(t1685: F, t4761: F, t8590: F, t4769: F, t4776: F, t6756: F, t6823: F, t8512: F, t8516: F, t8520: F, t8525: F, t8527: F, t8559: F, t8561: F, t8565: F, t8568: F, t8571: F) -> (F, F) {
    let t8592 = t4761 * t8590 * t1685;
    let t8607 = -0.1294625e1 * t8525 + 0.258925e1 * t8527 + t4769 + 0.20128333333333333334e0 * t6756 - 0.20128333333333333333e0 * t8512 + 0.60385e0 * t8516 - 0.301925e0 * t8520 + 0.82524375e-1 * t8559 + 0.16504875e0 * t8561 + t4776 + 0.11038e0 * t6823 - 0.27595e-1 * t8565 + 0.16557e0 * t8568 - 0.82785e-1 * t8571;
    (t8592, t8607)
}
