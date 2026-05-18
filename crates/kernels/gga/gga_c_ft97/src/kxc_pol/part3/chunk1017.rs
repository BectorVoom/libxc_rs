//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1017/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1017<F: Float>(t14895: F, t15118: F, t19004: F, t19008: F, t19013: F, t19018: F, t19022: F, t19025: F, t19028: F, t19032: F, t19243: F, t2755: F, t5362: F) -> (F, F) {
    let t19748 = -F::new(4.0) / F::new(9.0) * t19004 + F::new(4.0) / F::new(27.0) * t19008 - F::new(8.0) / F::new(27.0) * t14895 + t15118 + t19013 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t19018 - F::new(2.0) / F::new(9.0) * t19022 - F::new(2.0) / F::new(3.0) * t19025 - F::new(8.0) / F::new(9.0) * t19028 + t19032 / F::new(9.0) - t19243 / F::new(3.0);
    let t19752 = t2755 * t5362;
    (t19748, t19752)
}
