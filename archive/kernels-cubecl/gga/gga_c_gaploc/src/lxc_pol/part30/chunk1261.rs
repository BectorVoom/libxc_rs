//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1261/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1261<F: Float>(t10913: F, t10916: F, t1980: F, t14630: F, t3025: F, t948: F, t20157: F, t2085: F, t320: F, t32613: F, t1969: F, t3294: F, t5746: F, t8604: F) -> (F, F, F, F) {
    let t32853 = F::cast_from(0.42900587942220512002e1_f64) * t1980 * t10913 * t10916;
    let t32856 = F::cast_from(0.23833659967900284447e0_f64) * t3025 * t14630 * t948;
    let t32860 = F::cast_from(0.27606906686822939768e2_f64) * t320 * t2085 * t20157 * t32613;
    let t32866 = F::cast_from(0.12269736305254639897e2_f64) * t320 * t5746 * t20157 * t8604 * t3294 * t1969;
    (t32853, t32856, t32860, t32866)
}
