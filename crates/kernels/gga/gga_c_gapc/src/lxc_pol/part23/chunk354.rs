//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 354/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk354<F: Float>(t1647: F, t1649: F, t563: F, t589: F, t505: F, t597: F, t599: F, t561: F, t595: F, t198: F, t672: F, t674: F, t681: F) -> (F, F, F, F, F, F) {
    let t1650 = t1647 * t1649;
    let t1653 = t563 * t589;
    let t1659 = t597 * t505 * t599;
    let t1662 = t561 * t595;
    let t1665 = t672 * t198;
    let t1666 = t674 * t681;
    (t1650, t1653, t1659, t1662, t1665, t1666)
}
