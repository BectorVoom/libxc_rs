//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 694/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk694<F: Float>(t124: F, t6856: F, t123: F, t616: F, t1948: F, t6560: F, t121: F, t2057: F, t2061: F, t2064: F, t3411: F, t641: F, t642: F, t6843: F, t6847: F, t6850: F, t6855: F) -> (F, F, F, F, F) {
    let t6857 = t124 * t6856;
    let t6860 = t123 * t616;
    let t6861 = t6860 * t1948;
    let t6864 = t124 * t6560;
    let t6867 = -0.12897460341341234505e3 * t6843 * t121 * t124 + 0.11607714307207111054e4 * t6847 * t642 - 0.46430857228828444218e4 * t6850 * t2061 + 0.11607714307207111054e4 * t2057 * t2064 + 0.7738476204804740703e4 * t6855 * t6857 - 0.46430857228828444218e4 * t3411 * t6861 + 0.38692381024023703515e3 * t641 * t6864;
    (t6857, t6860, t6861, t6864, t6867)
}
