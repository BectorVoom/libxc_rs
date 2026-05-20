//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1286/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1286<F: Float>(t2113: F, t28271: F, t28277: F, t28974: F, t572: F, t7741: F, t26733: F, t129065: F, t129069: F, t129072: F, t129078: F, t129080: F, t2040: F, t28987: F, t32373: F, t5802: F, t7557: F, t7944: F, t8725: F) -> F {
    let t129082 = F::new(6.0) * t2113 * t28271;
    let t129084 = F::new(6.0) * t2113 * t28277;
    let t129089 = F::new(6.0) * t572 * t28974 * t7741;
    let t129092 = F::new(6.0) * t572 * t26733 * t7741;
    let t129093 = F::new(6.0) * t2040 * t28987 + F::new(6.0) * t5802 * t8725 + F::new(3.0) * t7557 * t7944 + t129065 + t129069 + t129072 + t129078 + t129080 + t129082 + t129084 + t129089 + t129092 + t32373;
    t129093
}
