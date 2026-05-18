//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1010/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1010<F: Float>(t12975: F, t19100: F, t25590: F, t25601: F, t25609: F, t30569: F, t30572: F, t30592: F, t30595: F, t30599: F, t30603: F, t1180: F) -> (F, F) {
    let t30605 = -t12975 - F::new(4.0) / F::new(9.0) * t19100 + F::new(2.0) / F::new(9.0) * t25590 - F::new(2.0) / F::new(3.0) * t25601 + t25609 / F::new(3.0) - F::new(10.0) / F::new(27.0) * t30592 + F::new(4.0) / F::new(3.0) * t30595 - F::new(2.0) / F::new(3.0) * t30569 - F::new(2.0) * t30599 + F::new(2.0) * t30572 - t30603 / F::new(3.0);
    let t30606 = t1180 * t30605;
    (t30605, t30606)
}
