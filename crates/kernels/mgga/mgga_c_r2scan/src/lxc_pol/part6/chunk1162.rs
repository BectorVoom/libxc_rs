//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1162/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1162<F: Float>(t5397: F, t5398: F, t611: F, t615: F, t616: F, t1759: F, t5468: F, t584: F, t410: F, t5926: F, t1986: F, t5455: F, t5456: F, t148: F, t166: F, t18944: F, t40: F, t591: F) -> (F, F, F, F, F, F) {
    let t21200 = 0.8129414272e-1 * t5397 * t611 * t615 * t616 * t5398;
    let t21202 = t584 * t5468 * t1759;
    let t21206 = t410 * t5926;
    let t21210 = 0.12467418556090653906e4 * t5455 * t1986 * t5456;
    let t21211 = t166 * t148;
    let t21216 = 0.33872559466666666667e-1 * t584 * t21211 * t18944 * t40 * t591;
    (t21200, t21202, t21206, t21210, t21211, t21216)
}
