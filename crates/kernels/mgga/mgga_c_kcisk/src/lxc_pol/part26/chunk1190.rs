//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1190/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1190<F: Float>(t2059: F, t2168: F, t32088: F, t3937: F, t27016: F, t2714: F, t6221: F, t9800: F, t2232: F, t33557: F, t415: F, t7907: F, t9469: F, t468: F, t7832: F, t32122: F, t8073: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t34766 = t2059 * t2168;
    let t34767 = t32088 * t34766;
    let t34768 = t3937 * t34767;
    let t34774 = t27016 * t2714;
    let t34777 = t6221 * t9800;
    let t34780 = t33557 * t2232;
    let t34781 = t415 * t34780;
    let t34783 = t9469 * t7907;
    let t34784 = t415 * t34783;
    let t34786 = t468 * t7832;
    let t34787 = t415 * t34786;
    let t34789 = t32122 * t8073;
    (t34767, t34768, t34774, t34777, t34780, t34781, t34783, t34784, t34786, t34787, t34789)
}
