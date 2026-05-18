//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 969/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk969<F: Float>(t15051: F, t2666: F, t10559: F, t10584: F, t10586: F, t10589: F, t10591: F, t10594: F, t10595: F, t10617: F, t10619: F, t13682: F, t13688: F, t15011: F, t15014: F, t15015: F, t15018: F, t15022: F, t15025: F, t15028: F, t15039: F, t15044: F, t15048: F, t462: F) -> F {
    let t15052 = t15051 * t2666;
    let t15055 = -F::new(4.0) / F::new(9.0) * t15011 + t15014 - F::new(22.0) / F::new(9.0) * t15015 - F::new(6.0) * t462 * t15018 + F::new(2.0) * t462 * t15022 - F::new(4.0) / F::new(27.0) * t15025 - t15028 - F::new(2.0) / F::new(9.0) * t10617 + t10559 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t10584 - F::new(8.0) / F::new(9.0) * t10595 - F::new(8.0) / F::new(27.0) * t10586 + t10589 / F::new(9.0) + F::new(2.0) / F::new(27.0) * t10591 - F::new(2.0) / F::new(9.0) * t10619 + F::new(4.0) * t462 * t15039 + F::new(4.0) / F::new(9.0) * t13682 * t15044 - F::new(4.0) / F::new(3.0) * t13688 * t15048 - F::new(4.0) / F::new(3.0) * t13688 * t15052 - t10594;
    t15055
}
