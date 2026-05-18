//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1270/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1270<F: Float>(t1300: F, t18433: F, t1884: F, t2233: F, t2272: F, t23379: F, t29657: F, t446: F, t448: F, t6260: F, t6896: F, t8014: F, t91791: F, t91793: F, t91863: F, t91866: F, t91869: F, t91872: F, t91874: F) -> F {
    let t101791 = -t91791 - t91793 - t91863 + t91866 - t91869 - t2233 * t1884 * t6260 / F::new(8.0) - t446 * t18433 * t2272 / F::new(16.0) - t2233 * t448 * t23379 / F::new(16.0) + t91872 - t91874 - t446 * t6896 * t8014 / F::new(16.0) - t446 * t1300 * t29657 / F::new(16.0);
    t101791
}
