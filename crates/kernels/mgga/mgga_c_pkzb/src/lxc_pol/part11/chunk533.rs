//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 533/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk533<F: Float>(t2782: F, t684: F, t664: F, t1083: F, t1901: F, t683: F, t1899: F, t1833: F, t1905: F, t2730: F, t2741: F, t1088: F, t694: F) -> (F, F, F, F, F, F, F) {
    let t2783 = t2782 * t684;
    let t2785 = F::new(1.0) * t664 * t2783;
    let t2786 = t1083 * t1901;
    let t2787 = t2786 * t683;
    let t2789 = F::new(0.16081979498692535067e2) * t1899 * t2787;
    let t2793 = t1905 - F::new(0.17123333333333333333e-1) * t1833 - F::new(0.17123333333333333333e-1) * t2730 + F::new(0.5137e-1) * t2741;
    let t2796 = t1088 * t694;
    (t2783, t2785, t2786, t2787, t2789, t2793, t2796)
}
