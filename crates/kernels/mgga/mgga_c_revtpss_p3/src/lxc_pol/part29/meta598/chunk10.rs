//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2037/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2037<F: Float>(t100988: F, t101012: F, t101055: F, t101083: F, t101099: F, t103570: F, t1940: F, t2403: F, t25781: F, t25784: F, t26425: F, t26581: F, t26585: F, t27770: F, t27777: F, t27802: F, t27810: F, t28456: F, t28460: F, t7200: F, t7428: F, t7432: F, t7862: F, t7869: F, t95511: F, t95527: F) -> F {
    let t103853 = F::new(3.0) * t2403 * t28456 * t7200 - t103570 + F::new(3.0) * t2403 * t7428 * t27777 - t1940 * t7432 * t101012 / F::new(2.0) - F::new(3.0) * t26425 * t101055 + F::new(3.0) * t2403 * t7428 * t27810 - t1940 * t28460 * t25784 / F::new(2.0) - t1940 * t26585 * t27802 - F::new(3.0) * t26425 * t101083 - t1940 * t28460 * t25781 + F::new(3.0) / F::new(2.0) * t2403 * t26581 * t7862 - F::new(3.0) * t95511 * t27770 - t1940 * t95527 * t7869 / F::new(2.0) - F::new(3.0) * t26425 * t100988 - t1940 * t7432 * t101099;
    t103853
}
