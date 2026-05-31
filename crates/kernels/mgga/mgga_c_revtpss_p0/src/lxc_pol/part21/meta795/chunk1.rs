//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2876/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2876<F: Float>(t11396: F, t4719: F, t15566: F, t5023: F, t52170: F, t52174: F, t52176: F, t52178: F, t52180: F, t52182: F, t52185: F, t52187: F, t52188: F, t52194: F) -> (F, F) {
    let t52196 = F::cast_from(0.51947577317044391277e2_f64) * t4719 * t11396;
    let t52197 = F::cast_from(6.0_f64) * t15566 * t5023 * t52188 + t52170 + t52174 - t52176 - t52178 + t52180 + t52182 - t52185 - t52187 + t52194 - t52196;
    (t52196, t52197)
}
