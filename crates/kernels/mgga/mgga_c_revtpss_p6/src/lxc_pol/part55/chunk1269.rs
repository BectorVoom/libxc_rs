//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1269/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1269<F: Float>(t2014: F, t32734: F, t5542: F, t4292: F, t651: F, t8686: F, t32385: F, t4248: F, t27123: F, t8641: F, t27126: F, t32401: F, t7732: F) -> (F, F, F, F, F, F) {
    let t128983 = t2014 * t32734 * t5542;
    let t128986 = t651 * t8686 * t4292;
    let t128988 = t4248 * t32385;
    let t128990 = t27123 * t8641;
    let t128992 = t27126 * t8641;
    let t128994 = t7732 * t32401;
    (t128983, t128986, t128988, t128990, t128992, t128994)
}
