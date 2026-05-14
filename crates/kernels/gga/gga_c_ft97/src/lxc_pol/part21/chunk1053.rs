//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1053/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1053<F: Float>(t3099: F, t422: F, t22522: F, t22572: F, t25704: F, t69: F, t8633: F, t22552: F, t22632: F, t25839: F, t5598: F, t6445: F, t92557: F, t25722: F, t5569: F, t1593: F, t5546: F) -> (F, F, F, F, F, F, F) {
    let t100645 = t422 * t3099;
    let t100667 = t22522 * t22572 * t25704;
    let t100678 = t69 * t8633;
    let t100697 = 0.51074886703703703704e-1 * t22552 * t22632 * t25839;
    let t100706 = t5598 * t92557 * t6445;
    let t100734 = 0.14846767889314528222e-3 * t5569 * t22572 * t25722;
    let t100737 = t5546 * t1593;
    (t100645, t100667, t100678, t100697, t100706, t100734, t100737)
}
