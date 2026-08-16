//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1145/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1145(t2490: f64, t2491: f64, t7627: f64, t160: f64, t805: f64, t91828: f64, t91830: f64, t91832: f64, t91835: f64, t91837: f64, t91839: f64, t91841: f64, t91844: f64, t91847: f64, t91850: f64, t91852: f64, t91854: f64) -> (f64, f64, f64) {
    let t91857 = t2490 * t7627 * t2491;
    let t91859 = t805 * t160;
    let t91861 = -3.0_f64 / 16.0_f64 * t91828 + t91830 / 8.0_f64 + 3.0_f64 / 2.0_f64 * t91832 + 15.0_f64 / 4.0_f64 * t91835 + 3.0_f64 / 32.0_f64 * t91837 - t91839 / 8.0_f64 - t91841 / 32.0_f64 - 3.0_f64 / 8.0_f64 * t91844 + 15.0_f64 / 8.0_f64 * t91847 + 3.0_f64 / 16.0_f64 * t91850 - 3.0_f64 * t91852 - 3.0_f64 / 4.0_f64 * t91854 + 3.0_f64 / 4.0_f64 * t91857 + 9.0_f64 / 4.0_f64 * t91859;
    (t91857, t91859, t91861)
}
