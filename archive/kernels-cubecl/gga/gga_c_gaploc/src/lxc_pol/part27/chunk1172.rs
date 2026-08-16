//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1172/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1172<F: Float>(t2268: F, t31585: F, t426: F, t535: F, t1222: F, t3344: F, t10262: F, t484: F, t1217: F, t3351: F, t2317: F, t6525: F, t7901: F) -> (F, F, F, F, F) {
    let t31685 = F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t535 * t31585 * t426;
    let t31687 = t1222 * t3344;
    let t31688 = F::cast_from(0.31616674039640166222e-2_f64) * t31687;
    let t31689 = t484 * t10262;
    let t31690 = F::cast_from(0.31616674039640166222e-2_f64) * t31689;
    let t31691 = t1217 * t3351;
    let t31692 = F::cast_from(0.36886119712913527259e-2_f64) * t31691;
    let t31694 = t6525 * t7901 * t2317;
    (t31685, t31688, t31690, t31692, t31694)
}
