//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 857/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk857<F: Float>(t595: F, t7006: F, t1669: F, t2799: F, t2461: F, t585: F, t159: F, t617: F, t1655: F, t2774: F, t5451: F, t5454: F, t5459: F, t5463: F, t5467: F, t5470: F, t5903: F, t598: F, t951: F) -> F {
    let t7768 = t595 * t7006;
    let t7776 = t2799 * t1669;
    let t7778 = t2461 * t585;
    let t7779 = t159 * t7778;
    let t7781 = F::cast_from(0.16936279733333333333e-2_f64) * t7779 * t617;
    let t7782 = -F::cast_from(0.675260332e-1_f64) * t7768 * t598 - F::cast_from(0.1350520664e0_f64) * t2774 * t1655 - F::cast_from(0.675260332e-1_f64) * t951 * t5903 + t5451 + t5454 - t5459 + t5463 + t5467 + F::cast_from(0.84681398666666666666e-3_f64) * t5470 - F::cast_from(0.11290853155555555555e-2_f64) * t7776 + t7781;
    t7782
}
