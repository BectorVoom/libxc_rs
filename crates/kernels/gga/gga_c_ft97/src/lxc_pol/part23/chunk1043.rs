//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1043/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1043<F: Float>(t2781: F, t31627: F, t1486: F, t193: F, t5299: F, t6222: F, t89: F, t1212: F, t28835: F, t25036: F, t25154: F, t28805: F, t28811: F, t31590: F, t31594: F, t31598: F, t31603: F, t31606: F, t31610: F, t31616: F, t31621: F, t31626: F) -> (F, F, F, F, F, F, F) {
    let t31628 = t2781 * t31627;
    let t31630 = t1486 * t193 * t31628;
    let t31631 = t6222 * t5299;
    let t31632 = t193 * t31631;
    let t31633 = t89 * t31632;
    let t31635 = t28835 * t1212;
    let t31636 = t193 * t31635;
    let t31637 = t89 * t31636;
    let t31639 = t31590 / 3.0 + t31594 / 6.0 + t31598 / 9.0 - t31603 - t31606 / 3.0 - 6.0 * t31610 - 4.0 / 3.0 * t28805 + 2.0 * t31616 + t31621 / 2.0 - t25036 - t25154 - 2.0 / 3.0 * t28811 - t31626 + t31630 + 2.0 * t31633 + 4.0 * t31637;
    (t31628, t31630, t31631, t31633, t31635, t31637, t31639)
}
