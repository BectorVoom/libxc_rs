//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 467/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk467<F: Float>(t1917: F, t703: F, t1830: F, t1880: F, t1833: F, t1845: F, t1863: F, t1868: F, t1874: F, t1876: F, t1883: F, t1887: F, t1891: F) -> (F, F, F, F) {
    let t1918 = t1917 * t703;
    let t1923 = 0.68863333333333333333e0 * t1830;
    let t1928 = 0.17365833333333333333e0 * t1880;
    let t1932 = -0.17648625e1 * t1863 + 0.3529725e1 * t1868 + t1923 - 0.103295e1 * t1833 + 0.1549425e1 * t1845 + 0.31558125e0 * t1874 + 0.6311625e0 * t1876 + t1928 - 0.41678e0 * t1883 + 0.312585e0 * t1887 + 0.312585e0 * t1891;
    (t1918, t1923, t1928, t1932)
}
