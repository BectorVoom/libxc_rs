//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 658/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk658<F: Float>(t640: F, t667: F, t1870: F, t618: F, t5: F, t515: F, t1784: F, t203: F, t674: F, t1946: F, t350: F, t1457: F, t563: F) -> (F, F, F, F, F, F) {
    let t5463 = t640 * t667;
    let t5479 = t618 * t1870;
    let t5486 = t515 * t5;
    let t5510 = t1784 * t674 * t203;
    let t5526 = t1946 * t350;
    let t5541 = t563 * t1457;
    (t5463, t5479, t5486, t5510, t5526, t5541)
}
