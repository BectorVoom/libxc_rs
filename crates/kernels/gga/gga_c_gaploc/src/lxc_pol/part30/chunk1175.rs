//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1175/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1175<F: Float>(t1: F, t106: F, t10667: F, t316: F, t28818: F, t28813: F, t28816: F, t28821: F, t28823: F, t28827: F, t32796: F, t33702: F, t33705: F, t33708: F, t33711: F, t33713: F, t33716: F, t33722: F, t6066: F, t7630: F, t780: F) -> (F,) {
    let t33725 = t10667 * t1 * t106 * t316;
    let t33728 = 0.63904876589867916128e-1 * t28818;
    let t33729 = t33702 + t33705 - t33708 + t33711 + t33713 + t33716 - 0.14300195980740170668e1 * t7630 * t6066 * t32796 - t33722 + 0.71500979903700853338e0 * t780 * t33725 + t28813 - t28816 + t33728 + t28821 - t28823 - t28827;
    (t33729,)
}
