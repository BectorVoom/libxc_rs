//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1098/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1098<F: Float>(t31570: F, t32866: F, t32867: F, t35764: F, t35790: F, t37714: F, t37717: F, t37718: F, t37721: F, t37722: F, t37723: F, t40145: F, t40147: F, t40152: F, t40156: F, t40158: F, t40163: F) -> (F,) {
    let t41973 = 0.62896184579208304138e-3 * t31570 - t35764 - t32866 - t32867 + t37714 + t37717 + t37718 + 0.34299214494455789578e-2 * t35790 - t37721 + t37722 + t37723 + 0.34299214494455789578e-2 * t40145 + 7.0 / 72.0 * t40147 + 0.21437009059034868486e-3 * t40152 + 0.14291339372689912324e-3 * t40156 - 0.62896184579208304138e-3 * t40158 - 0.41930789719472202759e-3 * t40163;
    (t41973,)
}
