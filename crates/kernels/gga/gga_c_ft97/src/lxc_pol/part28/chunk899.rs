//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 899/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk899<F: Float>(t147: F, t34978: F, t35237: F, t184: F, t1080: F, t21: F, t33234: F, t5: F, t7420: F, t920: F, t14: F, t7194: F, t72: F) -> (F, F, F, F, F) {
    let t148 = F::cast_from(10000000.0_f64) <= t147;
    let t35238 = t34978 + t35237;
    let t35239 = t35238 * t184;
    let t35247 = piecewise3::<F>(t148, F::new(0.0), t5 * t35239 * t21 / F::new(4.0) + t5 * t7420 * t920 / F::new(4.0) + t33234 * t1080 / F::new(4.0));
    let t36363 = t7194 * t14;
    let t36364 = t36363 * t72;
    (t35238, t35239, t35247, t36363, t36364)
}
