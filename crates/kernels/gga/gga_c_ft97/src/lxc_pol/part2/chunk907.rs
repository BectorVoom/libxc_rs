//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 907/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk907<F: Float>(t2526: F, t3977: F, t242: F, t10153: F, t1168: F, t13952: F, t13955: F, t13959: F, t13961: F, t13963: F, t13965: F, t13967: F, t14014: F, t14018: F, t14020: F, t14022: F, t14026: F, t14030: F, t1901: F, t446: F) -> (F, F, F) {
    let t14033 = t3977 * t2526;
    let t14034 = t242 * t14033;
    let t14037 = t10153 * t1168;
    let t14038 = t242 * t14037;
    let t14041 = F::new(2.0) / F::new(9.0) * t1901 * t13952 + F::new(2.0) / F::new(9.0) * t1901 * t13955 - t13959 - t13961 - t13963 + t13965 - F::new(2.0) / F::new(3.0) * t446 * t13967 - t446 * t14014 / F::new(3.0) - t14018 - t14020 - t446 * t14022 / F::new(9.0) - F::new(2.0) / F::new(27.0) * t446 * t14026 - F::new(2.0) / F::new(9.0) * t446 * t14030 - t446 * t14034 / F::new(3.0) - t446 * t14038 / F::new(3.0);
    (t14033, t14037, t14041)
}
