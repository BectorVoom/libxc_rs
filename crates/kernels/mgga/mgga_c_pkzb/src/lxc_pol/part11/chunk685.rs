//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 685/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk685<F: Float>(t1532: F, t4871: F, t1485: F, t557: F, t1531: F, t1639: F, t466: F, t1626: F, t496: F, t1541: F, t495: F, t127: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4872 = t4871 * t1532;
    let t4874 = t1485 * t557;
    let t4876 = F::cast_from(0.21687162600603479684e-1_f64) * t1531 * t4874;
    let t4877 = t466 * t1639;
    let t4879 = F::cast_from(0.32530743900905219526e-1_f64) * t1531 * t4877;
    let t4880 = t496 * t1626;
    let t4881 = F::cast_from(12.0_f64) * t4880;
    let t4882 = t495 * t1541;
    let t4883 = t4882 * t127;
    (t4872, t4874, t4876, t4877, t4879, t4880, t4881, t4882, t4883)
}
