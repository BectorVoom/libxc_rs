//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1169/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1169<F: Float>(t384: F, t45110: F, t22552: F, t22632: F, t25839: F, t1712: F, t938: F, t5598: F, t6445: F, t92557: F, t11253: F, t25797: F, t1751: F, t22572: F, t25722: F, t5569: F) -> (F, F, F, F, F, F, F) {
    let t100688 = t45110 * t384;
    let t100697 = 0.51074886703703703704e-1 * t22552 * t22632 * t25839;
    let t100698 = t938 * t1712;
    let t100706 = t5598 * t92557 * t6445;
    let t100708 = t25797 * t11253;
    let t100725 = t938 * t1751;
    let t100734 = 0.14846767889314528222e-3 * t5569 * t22572 * t25722;
    (t100688, t100697, t100698, t100706, t100708, t100725, t100734)
}
