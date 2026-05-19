//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1330/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1330<F: Float>(t14734: F, t2996: F, t2998: F, t3000: F, t5040: F, t5043: F, t5045: F, t5425: F, t6032: F, t6034: F, t6619: F, t6622: F, t6625: F) -> F {
    let t24708 = t14734 + F::new(64.0) * t2996 + F::new(120.0) * t2998 - F::new(16.0) * t3000 - F::cast_from(0.10389515463408878255e3_f64) * t5040 - F::cast_from(0.46785788981077169656e1_f64) * t5043 - F::cast_from(0.35089341735807877242e1_f64) * t5045 + F::new(8.0) * t6032 - F::new(8.0) * t6034 + F::new(12.0) * t6619 + F::new(24.0) * t5425 + F::new(12.0) * t6622 - F::new(4.0) * t6625;
    t24708
}
