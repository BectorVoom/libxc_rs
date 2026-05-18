//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 599/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk599<F: Float>(t2: F, t8326: F, t7794: F, t1775: F, t1788: F, t1793: F, t462: F, t8301: F, t8302: F, t8305: F, t8308: F, t8311: F, t8316: F, t8319: F, t8322: F, t8324: F, t92: F) -> (F, F, F) {
    let t8327 = t8326 * t2;
    let t8328 = t8327 * t7794;
    let t8331 = t1775 * t1788;
    let t8333 = t1775 * t1793;
    let t8335 = -t8301 - F::new(4.0) / F::new(3.0) * t8302 - t92 * t8305 - F::new(2.0) * t462 * t8308 + F::new(2.0) * t462 * t8311 + F::new(4.0) / F::new(3.0) * t462 * t8316 - F::new(2.0) / F::new(3.0) * t462 * t8319 + t462 * t8322 + t462 * t8324 + F::new(2.0) / F::new(3.0) * t462 * t8328 - F::new(2.0) / F::new(3.0) * t8331 - F::new(2.0) / F::new(3.0) * t8333;
    (t8327, t8328, t8335)
}
