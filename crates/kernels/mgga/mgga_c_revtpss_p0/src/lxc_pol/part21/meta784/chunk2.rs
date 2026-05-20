//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2823/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2823<F: Float>(t2430: F, t890: F, t14397: F, t1940: F, t2403: F, t2832: F, t40076: F, t40079: F, t40194: F, t40198: F, t4556: F, t50899: F, t50900: F, t50902: F, t50905: F, t50907: F) -> F {
    let t51806 = t2430 * t890;
    let t51810 = -F::new(3.0) * t14397 * t1940 * t2832 - F::new(9.0) * t2403 * t4556 * t51806 + t40076 - t40079 + t40194 + t40198 + t50899 - t50900 - t50902 + t50905 + t50907;
    t51810
}
