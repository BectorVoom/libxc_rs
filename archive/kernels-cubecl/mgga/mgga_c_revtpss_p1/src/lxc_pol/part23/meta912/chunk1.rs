//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2933/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2933<F: Float>(t63533: F, t63538: F, t63541: F, t63543: F, t63545: F, t63547: F, t63549: F, t63551: F, t77829: F, t77832: F, t77835: F, t77838: F) -> F {
    let t77961 = -F::cast_from(0.11577222222222222223e0_f64) * t63533 + F::cast_from(0.69463333333333333335e0_f64) * t63538 - F::cast_from(0.125034e1_f64) * t77829 + F::cast_from(0.62517e0_f64) * t77832 - F::cast_from(0.104195e0_f64) * t77835 - F::cast_from(0.104195e0_f64) * t77838 - F::cast_from(0.41678e0_f64) * t63541 + F::cast_from(0.69463333333333333333e-1_f64) * t63543 - F::cast_from(0.34731666666666666667e0_f64) * t63545 - F::cast_from(0.41678000000000000001e0_f64) * t63547 + F::cast_from(0.13892666666666666667e0_f64) * t63549 + F::cast_from(0.9261777777777777778e-1_f64) * t63551;
    t77961
}
