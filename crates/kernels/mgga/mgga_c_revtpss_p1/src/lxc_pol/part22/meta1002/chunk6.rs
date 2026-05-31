//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3415/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3415<F: Float>(t51909: F, t51911: F, t51913: F, t51915: F, t51917: F, t51921: F, t51923: F, t63238: F, t63240: F, t63242: F, t63246: F, t63250: F, t63255: F, t63260: F) -> F {
    let t64197 = -F::cast_from(0.55570666666666666668e0_f64) * t51909 + F::cast_from(0.9261777777777777778e-1_f64) * t51911 + F::cast_from(0.9261777777777777778e0_f64) * t51913 - F::cast_from(0.15436296296296296297e0_f64) * t51915 - F::cast_from(0.27785333333333333334e0_f64) * t51917 + F::cast_from(0.4630888888888888889e-1_f64) * t51921 + F::cast_from(0.61745185185185185187e-1_f64) * t51923 - F::cast_from(0.62517e0_f64) * t63238 + F::cast_from(0.83356000000000000001e0_f64) * t63240 - F::cast_from(0.55570666666666666667e0_f64) * t63242 - F::cast_from(0.62517e0_f64) * t63246 + F::cast_from(0.41678e0_f64) * t63250 + F::cast_from(0.41678e0_f64) * t63255 - F::cast_from(0.69463333333333333334e-1_f64) * t63260;
    t64197
}
