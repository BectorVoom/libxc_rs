//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1136/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1136<F: Float>(t5219: F, t5223: F, t1757: F, t5861: F, t615: F, t616: F, t1800: F, t5380: F, t5606: F, t1399: F, t5759: F, t1923: F) -> (F, F, F, F, F) {
    let t21396 = t5219 * t5223;
    let t21401 = 0.67745118933333333332e-2 * t1757 * t615 * t616 * t5861;
    let t21404 = 144.0 * t5380 * t5606 * t1800;
    let t21409 = 0.22911460125803964958e1 * t1399 * t5759;
    let t21416 = t1923 * t1923;
    (t21396, t21401, t21404, t21409, t21416)
}
