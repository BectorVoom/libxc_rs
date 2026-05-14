//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 935/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk935<F: Float>(t1564: F, t4458: F, t5675: F, t5674: F, t4606: F, t5691: F, t22958: F, t22953: F, t6495: F, t925: F, t22952: F, t23031: F, t4417: F, t446: F, t25899: F, t4462: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t29638 = t1564 * t5675 * t4458;
    let t29639 = t5674 * t29638;
    let t29641 = t5691 * t4606;
    let t29642 = t22958 * t29641;
    let t29643 = t5674 * t29642;
    let t29646 = t22953 * t6495 * t925;
    let t29647 = t22952 * t29646;
    let t29652 = t23031 * t4417;
    let t29653 = t1564 * t29652;
    let t29654 = t446 * t29653;
    let t29657 = t1564 * t25899 * t925;
    let t29658 = t5674 * t29657;
    let t29661 = t1564 * t5675 * t4462;
    (t29638, t29639, t29641, t29642, t29643, t29646, t29647, t29653, t29654, t29657, t29658, t29661)
}
