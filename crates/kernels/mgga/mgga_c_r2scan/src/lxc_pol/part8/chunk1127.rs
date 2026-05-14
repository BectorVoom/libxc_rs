//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1127/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1127<F: Float>(t5219: F, t5943: F, t1668: F, t1757: F, t1871: F, t615: F, t1684: F, t5402: F, t591: F, t5946: F, t5397: F, t5398: F, t611: F, t616: F, t1986: F, t5455: F, t5456: F) -> (F, F, F, F, F) {
    let t21186 = t5219 * t5943;
    let t21191 = 0.13549023786666666666e-1 * t1757 * t615 * t1668 * t1871;
    let t21195 = 0.13549023786666666666e-1 * t5946 * t1684 * t5402 * t591;
    let t21200 = 0.8129414272e-1 * t5397 * t611 * t615 * t616 * t5398;
    let t21210 = 0.12467418556090653906e4 * t5455 * t1986 * t5456;
    (t21186, t21191, t21195, t21200, t21210)
}
