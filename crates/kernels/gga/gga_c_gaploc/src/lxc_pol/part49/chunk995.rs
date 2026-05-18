//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 995/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk995<F: Float>(t1843: F, t32261: F, t7064: F, t2558: F, t33360: F, t9647: F, t13194: F, t1841: F, t13200: F, t13182: F, t29439: F, t13179: F, t7137: F) -> (F, F, F, F, F, F) {
    let t43090 = t7064 * t1843 * t32261;
    let t43093 = t9647 * t33360 * t2558;
    let t43094 = F::new(0.64087718584518535698e-3) * t43093;
    let t43095 = t1841 * t13194;
    let t43096 = F::new(0.17090058289204942852e-2) * t43095;
    let t43098 = t1841 * t13200;
    let t43099 = F::new(0.2563508743380741428e-2) * t43098;
    let t43100 = t29439 * t13182;
    let t43101 = F::new(0.64087718584518535698e-3) * t43100;
    let t43102 = t7137 * t13179;
    (t43090, t43094, t43096, t43099, t43101, t43102)
}
