//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1074/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1074<F: Float>(t11495: F, t1717: F, t1393: F, t3663: F, t9229: F, t11424: F, t563: F, t2983: F, t1787: F, t3684: F, t11381: F, t8787: F, t11463: F, t9330: F, t20171: F, t33287: F, t5708: F) -> (F, F, F, F, F, F, F, F) {
    let t34749 = t11495 * t1717;
    let t34752 = t1393 * t3663 * t9229;
    let t34754 = t563 * t11424;
    let t34755 = t34754 * t2983;
    let t34757 = t3684 * t1787;
    let t34759 = t8787 * t11381;
    let t34761 = t11463 * t9330;
    let t34764 = t5708 * t33287 * t20171;
    (t34749, t34752, t34754, t34755, t34757, t34759, t34761, t34764)
}
