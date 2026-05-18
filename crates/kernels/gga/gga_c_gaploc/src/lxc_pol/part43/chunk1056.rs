//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1056/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1056<F: Float>(t43479: F, t43489: F, t43497: F, t43500: F, t43511: F, t43514: F, t43523: F, t43527: F, t43567: F, t43569: F, t43571: F, t43575: F, t43579: F, t43582: F, t43592: F, t43597: F, t43601: F, t43602: F, t47234: F, t47245: F) -> F {
    let t51107 = -t43479 - t43489 + F::new(0.21450293971110256002e1) * t47234 - t43497 + t43500 - t43511 + t43514 + t43523 + t43527 - F::new(0.12423108009070322895e3) * t47245 + t43567 + t43569 + t43571 - t43575 + t43579 - t43582 + t43592 - t43597 + t43601 + t43602;
    t51107
}
