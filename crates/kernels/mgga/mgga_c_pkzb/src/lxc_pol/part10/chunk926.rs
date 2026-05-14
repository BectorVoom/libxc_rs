//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 926/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk926<F: Float>(t1676: F, t2706: F, t192: F, t2575: F, t135: F, t144: F, t1535: F, t1536: F, t1692: F, t2536: F, t2714: F, t2718: F, t5077: F, t5091: F, t5130: F, t5139: F, t5141: F, t560: F, t568: F, t637: F, t639: F, t6853: F, t7010: F, t7013: F, t7015: F, t7017: F, t7018: F, t7019: F, t7020: F, t7021: F, t7022: F, t7177: F) -> (F, F) {
    let t7181 = t2706 * t1676;
    let t7191 = t192 * t2575;
    let t7195 = 3.0 * t135 * t560 * t6853 + t135 * t144 * t7177 * t639 + t7010 + t5077 - t7013 + t7015 - t7017 - t7018 - t7019 + t5091 - 2.0 * t2536 * t7181 * t637 - t5130 - t7020 - t7021 + 3.0 * t1535 * t2714 * t1692 + 6.0 * t1535 * t1536 * t2575 + 12.0 * t2718 * t7191 * t568 - t5139 - t5141 + t7022;
    (t7181, t7195)
}
