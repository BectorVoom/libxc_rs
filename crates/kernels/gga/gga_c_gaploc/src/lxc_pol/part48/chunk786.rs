//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 786/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk786<F: Float>(t45423: F, t6066: F, t6111: F, t10914: F, t10915: F, t326: F, t45369: F, t825: F, t13588: F, t549: F, t11757: F, t9823: F, t11849: F, t2628: F, t43646: F, t43652: F) -> (F, F, F, F, F, F, F, F) {
    let t45426 = 0.42900587942220512003e1 * t6111 * t6066 * t45423;
    let t45429 = 0.21450293971110256001e1 * t10914 * t10915 * t45423;
    let t45432 = 0.18404604457881959845e2 * t825 * t326 * t45369;
    let t45437 = t6111 * t549 * t13588;
    let t45438 = 0.59584149919750711116e-1 * t45437;
    let t45440 = 0.35750489951850426669e0 * t9823 * t11757;
    let t45441 = t11849 * t2628;
    let t45442 = 0.29792074959875355558e-1 * t45441;
    let t45451 = 0.17875244975925213335e0 * t43646;
    let t45453 = 0.30674340763136599741e1 * t43652;
    (t45426, t45429, t45432, t45438, t45440, t45442, t45451, t45453)
}
