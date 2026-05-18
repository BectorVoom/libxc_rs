//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 482/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk482<F: Float>(t1830: F, t1880: F, t1833: F, t1845: F, t1863: F, t1868: F, t1874: F, t1876: F, t1883: F, t1887: F, t1891: F) -> (F, F, F) {
    let t1962 = F::new(0.40256666666666666667e0) * t1830;
    let t1967 = F::new(0.137975e0) * t1880;
    let t1971 = -F::new(0.1294625e1) * t1863 + F::new(0.258925e1) * t1868 + t1962 - F::new(0.60385e0) * t1833 + F::new(0.905775e0) * t1845 + F::new(0.82524375e-1) * t1874 + F::new(0.16504875e0) * t1876 + t1967 - F::new(0.33114e0) * t1883 + F::new(0.248355e0) * t1887 + F::new(0.248355e0) * t1891;
    (t1962, t1967, t1971)
}
