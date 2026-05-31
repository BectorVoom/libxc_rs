//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1302/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1302<F: Float>(t127495: F, t129069: F, t129072: F, t129078: F, t129080: F, t129082: F, t129084: F, t129089: F, t129092: F, t129095: F, t129097: F, t32373: F, t7696: F, t8127: F) -> F {
    let t131159 = F::cast_from(3.0_f64) * t7696 * t8127 + t127495 + t129069 + t129072 + t129078 + t129080 + t129082 + t129084 + t129089 + t129092 + t129095 + t129097 + t32373;
    t131159
}
