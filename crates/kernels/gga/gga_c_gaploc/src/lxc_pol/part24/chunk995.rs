//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 995/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk995<F: Float>(t124: F, t15478: F, t3307: F, t813: F, t10013: F, t2464: F, t2684: F, t10008: F, t825: F, t1402: F, t2033: F, t3280: F, t2628: F, t7403: F, t1980: F, t7634: F) -> (F, F, F, F, F, F, F) {
    let t28229 = t15478 * t124;
    let t28231 = t813 * t28229 * t3307;
    let t28242 = t2684 * t2464 * t10013;
    let t28245 = t825 * t2464 * t10008;
    let t28249 = 0.17875244975925213335e0 * t2033 * t1402 * t3280;
    let t28259 = 0.11916829983950142223e0 * t7403 * t2628;
    let t28279 = t1980 * t7634;
    (t28229, t28231, t28242, t28245, t28249, t28259, t28279)
}
