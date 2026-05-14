//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 879/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk879<F: Float>(t6050: F, t6828: F, t1417: F, t1127: F, t218: F, t709: F, t25057: F, t6776: F, t694: F, t226: F, t6762: F) -> (F, F, F, F, F, F, F) {
    let t27720 = t6828 * t6050;
    let t27721 = t1417 * t27720;
    let t27723 = t218 * t1127;
    let t27724 = t27723 * t709;
    let t27725 = t25057 * t27724;
    let t27729 = t694 * t6776;
    let t27730 = t27729 * t709;
    let t27733 = t6762 * t226;
    (t27720, t27721, t27723, t27725, t27729, t27730, t27733)
}
