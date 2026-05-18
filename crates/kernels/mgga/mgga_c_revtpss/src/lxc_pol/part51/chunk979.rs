//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 979/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk979<F: Float>(t1419: F, t3140: F, t8477: F, t25875: F, t32275: F, t32268: F, t32237: F, t2007: F, t7741: F, t651: F, t4248: F, t8461: F) -> (F, F, F, F, F, F, F, F) {
    let t32699 = t1419 * t3140;
    let t32700 = t8477 * t32699;
    let t32705 = t25875 * t32275;
    let t32710 = t32268 * t32275;
    let t32719 = t8477 * t32237;
    let t33574 = t2007 * t7741;
    let t33575 = t651 * t33574;
    let t33577 = t4248 * t8461;
    (t32699, t32700, t32705, t32710, t32719, t33574, t33575, t33577)
}
