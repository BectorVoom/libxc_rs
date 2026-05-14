//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 769/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk769<F: Float>(t1403: F, t1526: F, t2: F, t2320: F, t33545: F, t33557: F, t342: F, t343: F, t35757: F, t35761: F, t35766: F, t35772: F, t6895: F, t6900: F, t7426: F, t7427: F) -> (F,) {
    let t35777 = (-t35757 * t7427 / 6.0 + t33545 + t1403 * t35761 / 18.0 + t1403 * t6900 / 3.0 - t7426 * t35766 / 6.0 - t33557 - t1526 * t2320 * t6895 / 12.0 - t342 * t343 * t35772 / 4.0) * t2;
    (t35777,)
}
