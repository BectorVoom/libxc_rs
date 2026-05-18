//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 875/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk875<F: Float>(t15756: F, t3621: F, t2266: F, t4462: F, t643: F, t15768: F, t15763: F, t3613: F, t4454: F, t8654: F, t12143: F, t17549: F, t17552: F, t17554: F, t17556: F, t17560: F, t17564: F, t17569: F, t17573: F, t17577: F, t17583: F, t17586: F, t17590: F, t17593: F, t2265: F, t631: F, t8641: F, t8719: F) -> F {
    let t17595 = t3621 * t15756;
    let t17599 = t2266 * t4462 * t643;
    let t17602 = t3621 * t15768;
    let t17605 = t3613 * t15763;
    let t17609 = t8654 * t4454 * t643;
    let t17612 = -F::new(3.0) * t631 * t17549 + F::new(2.0) / F::new(9.0) * t17552 - t17554 / F::new(9.0) - t17556 / F::new(27.0) - F::new(3.0) / F::new(2.0) * t631 * t17560 + t631 * t17564 / F::new(6.0) + F::new(6.0) * t631 * t17569 + F::new(5.0) / F::new(27.0) * t8641 + F::new(4.0) / F::new(9.0) * t17573 - t2265 * t17577 / F::new(3.0) + F::new(5.0) / F::new(9.0) * t8719 + t2265 * t17583 - F::new(4.0) / F::new(3.0) * t12143 * t17586 + F::new(2.0) / F::new(3.0) * t2265 * t17590 + t2265 * t17593 - F::new(4.0) / F::new(3.0) * t12143 * t17595 - t2265 * t17599 / F::new(3.0) - t2265 * t17602 / F::new(3.0) + t2265 * t17605 / F::new(18.0) - t2265 * t17609 / F::new(9.0);
    t17612
}
