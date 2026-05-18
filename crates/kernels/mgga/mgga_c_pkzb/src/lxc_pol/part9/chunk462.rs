//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 462/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk462<F: Float>(t1885: F, t218: F, t219: F, t1843: F, t208: F, t1833: F, t1845: F, t1863: F, t1868: F, t1870: F, t1874: F, t1876: F, t1881: F, t1883: F) -> (F, F, F, F) {
    let t1887 = t218 * t219 * t1885;
    let t1889 = t208 * t1843;
    let t1891 = t218 * t219 * t1889;
    let t1893 = -F::new(0.9494625e0) * t1863 + F::new(0.1898925e1) * t1868 + t1870 - F::new(0.59793333333333333334e0) * t1833 + F::new(0.8969e0) * t1845 + F::new(0.15358125e0) * t1874 + F::new(0.3071625e0) * t1876 + t1881 - F::new(0.32862666666666666666e0) * t1883 + F::new(0.24647e0) * t1887 + F::new(0.24647e0) * t1891;
    (t1887, t1889, t1891, t1893)
}
