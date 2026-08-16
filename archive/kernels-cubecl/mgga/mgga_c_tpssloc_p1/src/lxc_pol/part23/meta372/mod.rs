//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta372<F: Float>(t3575: F, t42386: F, t11888: F, t11914: F, t11784: F, t820: F, t11779: F, t11153: F, t1176: F, t11881: F, t374: F, t485: F, t486: F, t9697: F) -> (F, F, F, F, F, F, F) {
        let (t45114, t45119, t45124, t45128, t45192, t45197, t45250) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1173::<F>(t3575, t42386, t11888, t11914, t11784, t820, t11779, t11153, t1176, t11881, t374, t485, t486, t9697);
    (t45114, t45119, t45124, t45128, t45192, t45197, t45250)
}
