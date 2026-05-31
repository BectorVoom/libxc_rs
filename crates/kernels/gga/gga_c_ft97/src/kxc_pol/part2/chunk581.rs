//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 581/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk581<F: Float>(t1168: F, t761: F, t684: F, t2606: F, t1901: F, t2549: F, t2553: F, t2554: F, t2556: F, t2584: F, t3281: F, t3835: F, t3839: F, t3844: F, t3848: F, t3852: F, t3856: F, t3861: F, t3866: F, t446: F) -> (F, F, F, F) {
    let t3869 = t761 * t1168;
    let t3870 = t3869 * t684;
    let t3871 = t2606 * t3870;
    let t3874 = t2584 / F::cast_from(27.0_f64) + t2554 / F::cast_from(9.0_f64) + t2556 / F::cast_from(9.0_f64) + t2553 - t2549 / F::cast_from(9.0_f64) + t3835 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t3839 + t446 * t3844 / F::cast_from(3.0_f64) - t446 * t3848 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3281 * t3852 - t446 * t3856 / F::cast_from(9.0_f64) + t446 * t3861 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t3866 + t1901 * t3871 / F::cast_from(9.0_f64);
    (t3869, t3870, t3871, t3874)
}
