//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1106/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1106<F: Float>(t1389: F, t246: F, t32247: F, t32275: F, t1381: F, t8590: F, t94801: F, t1032: F, t2022: F, t1955: F, t3140: F, t9656: F) -> (F, F, F, F, F, F) {
    let t121019 = t1389 * t246;
    let t121024 = t32247 * t32275;
    let t121028 = t94801 * t8590 * t1381;
    let t121030 = t2022 * t1032;
    let t121031 = t1955 * t121030;
    let t121034 = t3140 * t9656;
    (t121019, t121024, t121028, t121030, t121031, t121034)
}
