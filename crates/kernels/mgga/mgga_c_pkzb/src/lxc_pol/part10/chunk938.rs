//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 938/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk938<F: Float>(t12: F, t7335: F, t5528: F, t972: F, t1837: F, t8: F, t1429: F, t652: F, t1643: F, t1646: F, t2732: F, t2735: F, t6771: F, t82: F, zeta_threshold: F) -> (F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t7336 = 0.103295e1 * t7335;
    let t7337 = t5528 * t972;
    let t7340 = t1837 * t8;
    let t7345 = t652 * t1429;
    let t7350 = piecewise3(t84, 0.0, -28.0 / 27.0 * t7337 * t1643 + 16.0 / 9.0 * t7340 * t6771 + 4.0 / 9.0 * t2732 * t1646 - 2.0 / 3.0 * t7345 + 2.0 * t2735 * t82);
    (t7336, t7337, t7350)
}
