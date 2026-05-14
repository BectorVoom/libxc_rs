//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 800/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk800<F: Float>(t13625: F, t2684: F, t7354: F, t13626: F, t2013: F, t11724: F, t2464: F, t2465: F, t825: F, t43912: F, t2639: F, t3601: F, t7284: F, t787: F, t43914: F, t43917: F) -> (F, F, F, F, F, F, F) {
    let t45723 = t2684 * t7354 * t13625;
    let t45725 = t2013 * t13626;
    let t45729 = t825 * t2464 * t2465 * t11724;
    let t45731 = 0.11916829983950142223e0 * t43912;
    let t45735 = 0.53625734927775640005e1 * t787 * t7284 * t3601 * t2639;
    let t45736 = 0.59584149919750711116e-1 * t43914;
    let t45737 = 0.59584149919750711116e-1 * t43917;
    (t45723, t45725, t45729, t45731, t45735, t45736, t45737)
}
