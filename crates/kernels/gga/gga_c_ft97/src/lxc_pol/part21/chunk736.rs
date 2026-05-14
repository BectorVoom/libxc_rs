//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 736/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk736<F: Float>(t2266: F, t4458: F, t643: F, t15752: F, t3621: F, t15756: F, t4462: F, t15768: F, t15763: F, t3613: F, t4454: F, t8654: F, t12143: F, t17549: F, t17552: F, t17554: F, t17556: F, t17560: F, t17564: F, t17569: F, t17573: F, t17577: F, t17583: F, t17586: F, t2265: F, t631: F, t8641: F, t8719: F) -> (F,) {
    let t17590 = t2266 * t4458 * t643;
    let t17593 = t3621 * t15752;
    let t17595 = t3621 * t15756;
    let t17599 = t2266 * t4462 * t643;
    let t17602 = t3621 * t15768;
    let t17605 = t3613 * t15763;
    let t17609 = t8654 * t4454 * t643;
    let t17612 = -3.0 * t631 * t17549 + 2.0 / 9.0 * t17552 - t17554 / 9.0 - t17556 / 27.0 - 3.0 / 2.0 * t631 * t17560 + t631 * t17564 / 6.0 + 6.0 * t631 * t17569 + 5.0 / 27.0 * t8641 + 4.0 / 9.0 * t17573 - t2265 * t17577 / 3.0 + 5.0 / 9.0 * t8719 + t2265 * t17583 - 4.0 / 3.0 * t12143 * t17586 + 2.0 / 3.0 * t2265 * t17590 + t2265 * t17593 - 4.0 / 3.0 * t12143 * t17595 - t2265 * t17599 / 3.0 - t2265 * t17602 / 3.0 + t2265 * t17605 / 18.0 - t2265 * t17609 / 9.0;
    (t17612,)
}
