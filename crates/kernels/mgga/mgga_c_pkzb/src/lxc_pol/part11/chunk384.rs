//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 384/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk384<F: Float>(t1571: F, t526: F, t1480: F, t1483: F, t1486: F, t1490: F, t1492: F, t1495: F) -> (F, F) {
    let t1572 = t1571 * t526;
    let t1581 = -F::new(0.78438333333333333333e0) * t1480 + F::new(0.15687666666666666667e1) * t1483 + F::new(0.68863333333333333333e0) * t1486 + F::new(0.14025833333333333333e0) * t1490 + F::new(0.28051666666666666667e0) * t1492 + F::new(0.17365833333333333333e0) * t1495;
    (t1572, t1581)
}
