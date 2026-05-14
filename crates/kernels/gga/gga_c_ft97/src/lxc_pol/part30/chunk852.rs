//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 852/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk852<F: Float>(t142832: F, t19038: F, t287: F, t7464: F, t39: F, t40: F, t4113: F, t43585: F, t1466: F, t34024: F, t681: F, t7584: F, t880: F, t34265: F, t6210: F, t34325: F) -> (F, F, F, F, F, F, F) {
    let t142833 = t19038 * t142832;
    let t142834 = t7464 * t287;
    let t142867 = t4113 * t43585 * t39 * t40;
    let t142911 = t1466 * t681 * t34024;
    let t142913 = t7584 * t880;
    let t142918 = t6210 * t34265;
    let t142925 = t1466 * t681 * t34325;
    (t142833, t142834, t142867, t142911, t142913, t142918, t142925)
}
