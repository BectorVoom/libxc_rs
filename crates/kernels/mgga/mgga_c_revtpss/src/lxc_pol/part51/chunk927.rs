//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 927/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk927<F: Float>(t31991: F, t94121: F, t25610: F, t31949: F, t1035: F, t1061: F, t1078: F, t31897: F, t3173: F, t32000: F, t8513: F, t93469: F, t11627: F, t3148: F, t31998: F, t31903: F) -> (F, F, F, F, F, F, F, F) {
    let t120218 = t94121 * t31991;
    let t120223 = t25610 * t31949;
    let t120237 = t1078 * t1035 * t1061;
    let t120238 = t31897 * t120237;
    let t120244 = t32000 * t3173;
    let t120248 = t8513 * t93469;
    let t120251 = t120248 * t1078 * t11627 * t3148;
    let t120256 = t120248 * t31998 * t3148;
    let t120259 = t31903 * t120237;
    (t120218, t120223, t120237, t120238, t120244, t120251, t120256, t120259)
}
