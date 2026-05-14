//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 865/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk865<F: Float>(t1100: F, t1416: F, t1113: F, t218: F, t709: F, t24345: F, t6050: F, t6828: F, t1417: F, t1127: F, t25057: F, t6776: F, t694: F, t226: F, t6762: F, t3817: F, t6018: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t27711 = t1100 * t1416;
    let t27712 = t218 * t1113;
    let t27713 = t27712 * t709;
    let t27717 = t1100 * t24345;
    let t27720 = t6828 * t6050;
    let t27721 = t1417 * t27720;
    let t27723 = t218 * t1127;
    let t27724 = t27723 * t709;
    let t27725 = t25057 * t27724;
    let t27729 = t694 * t6776;
    let t27730 = t27729 * t709;
    let t27733 = t6762 * t226;
    let t27736 = t6018 * t3817;
    (t27711, t27712, t27713, t27717, t27720, t27721, t27723, t27725, t27729, t27730, t27733, t27736)
}
