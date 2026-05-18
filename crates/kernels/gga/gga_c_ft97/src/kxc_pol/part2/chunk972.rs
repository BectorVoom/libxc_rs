//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 972/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk972<F: Float>(t15073: F, t871: F, t14635: F, t14637: F, t14639: F, t14619: F, t14622: F, t14626: F, t14630: F, t14633: F, t14642: F, t14645: F, t14650: F) -> (F, F) {
    let t15074 = t871 * t15073;
    let t15081 = F::new(2.0) / F::new(27.0) * t14635;
    let t15082 = F::new(4.0) / F::new(27.0) * t14637;
    let t15083 = F::new(4.0) / F::new(81.0) * t14639;
    let t15087 = -F::new(8.0) / F::new(9.0) * t14619 + F::new(8.0) / F::new(27.0) * t14622 + t14626 / F::new(9.0) - F::new(4.0) / F::new(9.0) * t14630 + F::new(2.0) / F::new(9.0) * t14633 - t15081 - t15082 + t15083 - F::new(2.0) / F::new(27.0) * t14642 - F::new(10.0) / F::new(81.0) * t14645 - F::new(2.0) / F::new(9.0) * t14650;
    (t15074, t15087)
}
