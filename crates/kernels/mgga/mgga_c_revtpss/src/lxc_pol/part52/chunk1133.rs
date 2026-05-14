//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1133/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1133<F: Float>(t4292: F, t651: F, t8686: F, t32385: F, t4248: F, t27123: F, t8641: F, t27126: F, t32401: F, t7732: F, t128970: F, t128974: F, t128975: F, t128977: F, t128979: F, t128981: F, t128983: F, t1453: F, t28927: F, t34326: F, t8568: F) -> (F,) {
    let t128986 = t651 * t8686 * t4292;
    let t128988 = t4248 * t32385;
    let t128990 = t27123 * t8641;
    let t128992 = t27126 * t8641;
    let t128994 = t7732 * t32401;
    let t128997 = t1453 * t34326 + t28927 * t8568 + t128970 - t128974 + t128975 - t128977 - t128979 - t128981 - t128983 - 2.0 * t128986 - 2.0 * t128988 - 2.0 * t128990 - 2.0 * t128992 - 2.0 * t128994;
    (t128997,)
}
