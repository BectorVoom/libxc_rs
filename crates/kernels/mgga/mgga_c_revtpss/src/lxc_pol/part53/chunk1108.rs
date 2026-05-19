//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1108/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1108<F: Float>(t121072: F, t25304: F, t32217: F, t8477: F, t8705: F, t9656: F, t3999: F, t8578: F, t25880: F, t676: F, t7274: F, t32705: F) -> (F, F, F, F, F) {
    let t121074 = F::cast_from(0.45699670022203476294e-2_f64) * t25304 * t32217 * t121072;
    let t121076 = t8477 * t8705 * t9656;
    let t121077 = t3999 * t8578;
    let t121086 = t25880 * t676 * t7274;
    let t121087 = t32705 * t121086;
    (t121074, t121076, t121077, t121086, t121087)
}
