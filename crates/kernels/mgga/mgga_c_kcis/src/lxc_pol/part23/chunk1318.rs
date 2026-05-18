//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1318/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1318<F: Float>(t1300: F, t2132: F, t2233: F, t27325: F, t28876: F, t3707: F, t3708: F, t446: F, t5407: F, t8014: F, t8130: F, t8255: F, t91885: F, t91895: F, t91901: F, t92157: F, t92379: F, t97561: F) -> F {
    let t99758 = -t91885 - t446 * t1300 * t28876 / F::new(8.0) + t97561 - t2233 * t3707 * t2132 / F::new(16.0) + t91895 - t91901 + t92379 - t446 * t3708 * t8255 / F::new(16.0) + t92157 - t446 * t5407 * t8014 / F::new(8.0) + t8130 * t27325 / F::new(16.0);
    t99758
}
