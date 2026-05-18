//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 860/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk860<F: Float>(t833: F, t8746: F, t3199: F, t376: F, t829: F, t830: F, t3062: F, t4414: F, t4395: F, t8652: F, t3074: F, t2379: F, t3083: F) -> (F, F, F, F, F, F) {
    let t8747 = t8746 * t833;
    let t8749 = t3199 * t376;
    let t8751 = t829 * t830 * t8749;
    let t8771 = F::new(7.0) / F::new(72.0) * t4414 * t3062;
    let t8775 = t4395 * t8652;
    let t8776 = t3074 * t8775;
    let t8780 = F::new(7.0) / F::new(144.0) * t3083 * t2379;
    (t8747, t8749, t8751, t8771, t8776, t8780)
}
