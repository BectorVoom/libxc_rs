//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1082/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1082<F: Float>(t11269: F, t1526: F, t1527: F, t15567: F, t16633: F, t16640: F, t20022: F, t20031: F, t20039: F, t20545: F, t20556: F, t20560: F, t20568: F, t3088: F, t41318: F, t41349: F, t78678: F, t78681: F, t8766: F) -> F {
    let t87285 = -t1526 * t3088 * t20545 / F::new(3.0) - F::new(7.0) / F::new(27.0) * t1526 * t11269 * t41318 * t20022 - t1526 * t1527 * t20560 / F::new(4.0) - t1526 * t1527 * t8766 * t20022 / F::new(2.0) + t15567 * t16640 * t20039 / F::new(2.0) + t1526 * t1527 * t20568 / F::new(2.0) - t1526 * t1527 * t20556 / F::new(4.0) - t78678 / F::new(9.0) - t78681 / F::new(6.0) + F::new(2.0) / F::new(3.0) * t1526 * t3088 * t41349 * t20022 - t15567 * t16633 * t20031 / F::new(3.0);
    t87285
}
