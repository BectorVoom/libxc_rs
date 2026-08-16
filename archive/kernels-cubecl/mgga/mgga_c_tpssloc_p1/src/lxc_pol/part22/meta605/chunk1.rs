//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2130/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2130<F: Float>(t49922: F, t10402: F, t14618: F, t14608: F, t10936: F, t4669: F, t3082: F, t4617: F, t1041: F, t4584: F, t49850: F, t14159: F, t2960: F) -> (F, F, F, F, F, F, F) {
    let t49923 = t49922 / F::cast_from(2304.0_f64);
    let t49929 = t14618 * t10402;
    let t49934 = t14608 * t10402;
    let t49984 = t4669 * t10936;
    let t49993 = t4617 * t3082;
    let t49994 = t49993 / F::cast_from(4608.0_f64);
    let t50047 = t1041 * t49850 * t4584;
    let t50048 = t50047 / F::cast_from(3456.0_f64);
    let t50077 = t2960 * t14159;
    (t49923, t49929, t49934, t49984, t49994, t50048, t50077)
}
