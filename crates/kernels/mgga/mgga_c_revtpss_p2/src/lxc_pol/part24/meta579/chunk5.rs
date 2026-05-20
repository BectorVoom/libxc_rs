//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1790/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1790<F: Float>(t24677: F, t467: F, t475: F, t484: F, t52: F, t6594: F, t6601: F, t71187: F, t71192: F, t83849: F, t83851: F, t83860: F, t83863: F, t83871: F, t83891: F, t83897: F, rho1: F) -> F {
    let t91303 = F::cast_from(0.18292914397043087775e-1_f64) * t83849 + F::cast_from(0.34299214494455789578e-2_f64) * t83851 - F::cast_from(0.19055119163586549765e-2_f64) * t83860 + F::cast_from(0.57165357490759649296e-3_f64) * t83863 - F::cast_from(0.22866142996303859719e-2_f64) * t83871 - F::cast_from(0.17149607247227894789e-2_f64) * t83891 - F::cast_from(0.22866142996303859719e-2_f64) * t83897 + F::cast_from(0.30488190661738479624e-2_f64) * t71187 - F::cast_from(0.28582678745379824648e-3_f64) * t71192 + F::cast_from(0.4425022116877321001e0_f64) * t467 * t475 / t52 / t24677 / rho1 * t484 + F::cast_from(0.43445671692977333464e-1_f64) * t6601 * t6594 * t484;
    t91303
}
