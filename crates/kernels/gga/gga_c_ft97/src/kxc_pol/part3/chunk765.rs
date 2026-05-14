//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 765/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk765<F: Float>(t4454: F, t643: F, t8654: F, t12143: F, t17549: F, t17552: F, t17554: F, t17556: F, t17560: F, t17564: F, t17569: F, t17573: F, t17577: F, t17583: F, t17586: F, t17590: F, t17593: F, t17595: F, t17599: F, t17602: F, t17605: F, t2265: F, t631: F, t8641: F, t8719: F) -> (F,) {
    let t17609 = t8654 * t4454 * t643;
    let t17612 = -3.0 * t631 * t17549 + 2.0 / 9.0 * t17552 - t17554 / 9.0 - t17556 / 27.0 - 3.0 / 2.0 * t631 * t17560 + t631 * t17564 / 6.0 + 6.0 * t631 * t17569 + 5.0 / 27.0 * t8641 + 4.0 / 9.0 * t17573 - t2265 * t17577 / 3.0 + 5.0 / 9.0 * t8719 + t2265 * t17583 - 4.0 / 3.0 * t12143 * t17586 + 2.0 / 3.0 * t2265 * t17590 + t2265 * t17593 - 4.0 / 3.0 * t12143 * t17595 - t2265 * t17599 / 3.0 - t2265 * t17602 / 3.0 + t2265 * t17605 / 18.0 - t2265 * t17609 / 9.0;
    (t17612,)
}
