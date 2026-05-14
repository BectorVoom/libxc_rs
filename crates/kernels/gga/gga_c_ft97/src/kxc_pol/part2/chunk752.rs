//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 752/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk752<F: Float>(t13346: F, t3917: F, t13293: F, t13297: F, t13302: F, t13306: F, t13308: F, t13310: F, t13316: F, t13321: F, t13325: F, t13329: F, t13332: F, t13335: F, t13338: F, t13339: F, t13345: F, t3139: F, t462: F, t9905: F, t9933: F, t9936: F, t9962: F) -> (F,) {
    let t13347 = t3917 * t13346;
    let t13350 = -4.0 / 3.0 * t3139 * t13293 + 2.0 / 3.0 * t462 * t13297 - 8.0 / 3.0 * t3139 * t13302 - t13306 + t13308 - 2.0 / 9.0 * t462 * t13310 - 10.0 / 27.0 * t462 * t13316 + 8.0 / 9.0 * t3139 * t13321 + t462 * t13325 / 3.0 - t13329 - 2.0 / 9.0 * t9905 + 2.0 * t462 * t13332 - 4.0 / 27.0 * t13335 - t13338 - 22.0 / 9.0 * t13339 + t9933 / 3.0 - 8.0 / 9.0 * t9936 - 2.0 / 3.0 * t9962 + t13345 - 2.0 * t462 * t13347;
    (t13350,)
}
