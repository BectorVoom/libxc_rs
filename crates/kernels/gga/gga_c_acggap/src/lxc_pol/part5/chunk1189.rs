//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1189/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1189<F: Float>(t14729: F, t14731: F, t14732: F, t5414: F, t5417: F, t5419: F, t5422: F, t6022: F, t6025: F, t6604: F, t6607: F, t6612: F, t6616: F, t14734: F, t2996: F, t2998: F, t3000: F, t5040: F, t5043: F, t5045: F, t5425: F, t6032: F, t6034: F, t6619: F, t6622: F, t6625: F) -> (F, F) {
    let t24695 = 12.0 * t5414 - 2.0 * t5417 + 6.0 * t6604 + 6.0 * t5419 + 12.0 * t6607 + 6.0 * t5422 - t14729 + t14731 - t14732 + 12.0 * t6612 - 0.11696447245269292414e1 * t6022 - 2.0 * t6616 - 0.36622894612013090108e-3 * t6025;
    let t24708 = t14734 + 64.0 * t2996 + 120.0 * t2998 - 16.0 * t3000 - 0.10389515463408878255e3 * t5040 - 0.46785788981077169656e1 * t5043 - 0.35089341735807877242e1 * t5045 + 8.0 * t6032 - 8.0 * t6034 + 12.0 * t6619 + 24.0 * t5425 + 12.0 * t6622 - 4.0 * t6625;
    (t24695, t24708)
}
