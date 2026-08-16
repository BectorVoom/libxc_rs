//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 913/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk913<F: Float>(t487: F, t5617: F, t23339: F, t47660: F, t1786: F, t5710: F, t5704: F, t38456: F, t91: F, t1608: F, t1689: F, t5584: F) -> (F, F, F, F, F, F) {
    let t91583 = t487 * t5617;
    let t91739 = t47660 * t23339;
    let t91771 = t1786 * t5710;
    let t92049 = t1786 * t5704;
    let t92173 = t91 * t38456;
    let t92278 = t1608 * t5584 * t1689;
    (t91583, t91739, t91771, t92049, t92173, t92278)
}
