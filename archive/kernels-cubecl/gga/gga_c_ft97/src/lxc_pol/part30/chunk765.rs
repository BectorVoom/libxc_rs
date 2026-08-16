//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 765/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk765<F: Float>(t242: F, t33602: F, t713: F, t7546: F, t2568: F, t729: F, t33599: F, t1882: F, t7495: F, t7499: F, t33264: F, t10002: F) -> (F, F, F, F, F, F, F, F) {
    let t33668 = t242 * t33602;
    let t33671 = t7546 * t713;
    let t33673 = t729 * t2568 * t33671;
    let t33676 = t242 * t33599;
    let t33680 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t7495;
    let t33682 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t7499;
    let t33683 = t242 * t33264;
    let t33686 = t10002 * t7546;
    (t33668, t33671, t33673, t33676, t33680, t33682, t33683, t33686)
}
