//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 830/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk830<F: Float>(t147: F, t32741: F, t33229: F, t184: F, t5: F, t7419: F, t21: F, t363: F, t650: F, t7420: F, t1337: F, t942: F, t5507: F) -> (F, F, F, F, F, F) {
    let t148 = F::new(10000000.0) <= t147;
    let t33230 = t32741 + t33229;
    let t33231 = t33230 * t184;
    let t33234 = t5 * t7419;
    let t33240 = piecewise3::<f64>(t148, F::new(0.0), t5 * t33231 * t21 / F::new(4.0) + t5 * t7420 * t363 / F::new(4.0) + t33234 * t650 / F::new(4.0));
    let t34352 = t1337 * t942;
    let t34353 = t5507 * t34352;
    (t33230, t33231, t33234, t33240, t34352, t34353)
}
