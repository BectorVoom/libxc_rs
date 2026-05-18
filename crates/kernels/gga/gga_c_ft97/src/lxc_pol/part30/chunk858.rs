//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 858/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk858<F: Float>(t35760: F, t666: F, t461: F, t6903: F, t231: F, t6837: F, t1403: F, t1526: F, t2: F, t2320: F, t33545: F, t33557: F, t342: F, t343: F, t35757: F, t6895: F, t6900: F, t7426: F, t7427: F) -> (F, F, F, F) {
    let t35761 = t666 * t35760;
    let t35766 = t461 * t6903;
    let t35772 = t231 * t6837;
    let t35777 = (-t35757 * t7427 / F::new(6.0) + t33545 + t1403 * t35761 / F::new(18.0) + t1403 * t6900 / F::new(3.0) - t7426 * t35766 / F::new(6.0) - t33557 - t1526 * t2320 * t6895 / F::new(12.0) - t342 * t343 * t35772 / F::new(4.0)) * t2;
    (t35761, t35766, t35772, t35777)
}
