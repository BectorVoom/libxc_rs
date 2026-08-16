//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2914/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2914<F: Float>(t300: F, t52368: F, t15547: F, t3030: F, t3012: F, t1634: F, t52239: F, t15520: F, t3022: F, t52481: F, t52486: F, t52488: F, t52490: F, t52492: F, t52495: F, t52499: F) -> (F, F, F, F, F) {
    let t52874 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t52368;
    let t52876 = F::cast_from(0.17544670867903938621e1_f64) * t15547 * t3030;
    let t52877 = t300 * t3012;
    let t52880 = F::cast_from(0.10526802520742363173e2_f64) * t52877 * t1634 * t52239;
    let t52882 = F::cast_from(0.35089341735807877242e1_f64) * t3022 * t15520;
    let t52883 = t52481 + t52874 - t52876 - t52880 + t52486 + t52882 + t52488 - t52490 + t52492 - t52495 + t52499;
    (t52874, t52876, t52880, t52882, t52883)
}
