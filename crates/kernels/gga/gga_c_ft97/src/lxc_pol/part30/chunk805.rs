//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 805/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk805<F: Float>(t6287: F, t6353: F, t840: F, t25188: F, t6374: F, t296: F, t1501: F, t6260: F, t871: F, t1882: F, t7681: F, t1476: F, t6393: F) -> (F, F, F, F, F, F, F) {
    let t34111 = t840 * t6353 * t6287;
    let t34114 = t25188 * t6374;
    let t34115 = t296 * t34114;
    let t34118 = t6260 * t1501;
    let t34120 = t840 * t871 * t34118;
    let t34124 = t1882 * t7681 / F::cast_from(9.0_f64);
    let t34126 = t840 * t6393 * t1476;
    (t34111, t34114, t34115, t34118, t34120, t34124, t34126)
}
