//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1242/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1242<F: Float>(t4702: F, t53: F, t5591: F, t72: F, t1701: F, t22652: F, t105007: F, t115486: F, t118930: F, t119001: F, t23869: F, t26678: F, t30064: F, t4699: F, t538: F, t5579: F, t5790: F, t5838: F, t61672: F, t76883: F, t8812: F, t8838: F, t94376: F, t94387: F, t94838: F, t94936: F) -> (F, F) {
    let t119022 = t5591 * t72 * t4702 * t53;
    let t119046 = t1701 * t22652 * t4702;
    let t119053 = 0.72503285312204600893e0 * t94376 * t119022 - 0.72503285312204600893e0 * t94838 * t119022 + 0.40006800655555555556e0 * t105007 * t30064 - 0.16669500273148148149e-1 * t5838 * t115486 - 0.60010200983333333334e0 * t94387 * t5579 * t72 * t4702 * t538 + 0.12002040196666666667e1 * t94936 * t5579 * t72 * t76883 - 0.54738951849294959987e0 * t8812 * t5790 * t4699 + 0.90613700826057446696e0 * t61672 * t26678 - 0.24163653553615319119e1 * t8838 * t119046 + 0.90613700826057446696e0 * t8838 * t118930 + 0.45306850413028723348e0 * t23869 * t119001;
    (t119046, t119053)
}
