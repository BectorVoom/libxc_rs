//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1292/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1292<F: Float>(t32099: F, t33952: F, t33966: F, t33968: F, t33974: F, t33979: F, t33997: F, t34008: F, t34012: F, t34018: F, t34023: F, t35240: F, t38876: F, t38880: F, t38881: F, t39339: F, t39342: F, t39519: F, t39523: F, t39551: F) -> (F,) {
    let t39579 = t32099 - t38876 + t33952 + t33966 - t33968 - t33974 - t33979 + t33997 + t34008 + t34012 + t38880 - t38881 + t39523 - t34018 + t34023 + t39339 + 2.0 * t39551 - t39342 - t39519 - t35240;
    (t39579,)
}
