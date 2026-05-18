//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 846/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk846<F: Float>(t2575: F, t2719: F, t3491: F, t639: F, t192: F, t3396: F, t135: F, t144: F, t1535: F, t2536: F, t2537: F, t2706: F, t2714: F, t2718: F, t4996: F, t5005: F, t5011: F, t5019: F, t5022: F, t5154: F, t560: F, t568: F, t7030: F, t7037: F, t7042: F, t8795: F, t8817: F, t8842: F, t8843: F, t8844: F, t9099: F) -> (F, F, F) {
    let t9103 = t2719 * t2575;
    let t9112 = t3491 * t639;
    let t9116 = t192 * t3396;
    let t9120 = t135 * t144 * t639 * t9099 + F::new(3.0) * t135 * t560 * t8817 + F::new(6.0) * t1535 * t2575 * t2714 + F::new(3.0) * t1535 * t568 * t9112 - F::new(2.0) * t2536 * t2537 * t2706 + F::new(6.0) * t2718 * t568 * t9116 + F::new(12.0) * t2718 * t9103 + t4996 + t5005 - t5011 + t5019 - t5022 - t5154 + t7030 - t7037 - t7042 - t8795 - t8842 - t8843 + t8844;
    (t9112, t9116, t9120)
}
