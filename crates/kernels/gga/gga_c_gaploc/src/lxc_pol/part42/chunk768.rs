//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 768/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk768<F: Float>(t11765: F, t9823: F, t2536: F, t3614: F, t2009: F, t2021: F, t2684: F, t45320: F, t7585: F, t45369: F, t7427: F, t7573: F, t10930: F, t10931: F, t326: F, t45423: F, t825: F) -> (F, F, F, F, F, F) {
    let t45755 = 0.35750489951850426669e0 * t9823 * t11765;
    let t45758 = t2536 * t3614;
    let t45761 = 0.35750489951850426669e0 * t2021 * t45758 * t2009;
    let t45766 = 0.87421871174939309262e2 * t2684 * t7585 * t45320;
    let t45772 = 0.37959496694381542179e3 * t7427 * t7573 * t45369;
    let t45775 = 0.38649669361552115674e3 * t10930 * t10931 * t45369;
    let t45778 = 0.92023022289409799224e1 * t825 * t326 * t45423;
    (t45755, t45761, t45766, t45772, t45775, t45778)
}
