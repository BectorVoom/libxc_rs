//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 374/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk374<F: Float>(t2568: F, t3864: F, t242: F, t1168: F, t761: F, t684: F, t2606: F, t1901: F, t2549: F, t2553: F, t2554: F, t2556: F, t2584: F, t3281: F, t3835: F, t3839: F, t3844: F, t3848: F, t3852: F, t3856: F, t3861: F, t446: F) -> (F, F, F, F, F) {
    let t3865 = t2568 * t3864;
    let t3866 = t242 * t3865;
    let t3869 = t761 * t1168;
    let t3870 = t3869 * t684;
    let t3871 = t2606 * t3870;
    let t3874 = t2584 / 27.0 + t2554 / 9.0 + t2556 / 9.0 + t2553 - t2549 / 9.0 + t3835 / 27.0 + 2.0 / 3.0 * t446 * t3839 + t446 * t3844 / 3.0 - t446 * t3848 / 9.0 + 2.0 / 9.0 * t3281 * t3852 - t446 * t3856 / 9.0 + t446 * t3861 / 3.0 + 2.0 / 3.0 * t446 * t3866 + t1901 * t3871 / 9.0;
    (t3866, t3869, t3870, t3871, t3874)
}
