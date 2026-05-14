//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 553/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk553<F: Float>(t1788: F, t1803: F, t1806: F, t1808: F, t1812: F, t1816: F, t1825: F, t1829: F, t1833: F, t1840: F, t1844: F, t1851: F, t2763: F, t2765: F, t2780: F, t1856: F, t1858: F, t1874: F, t1885: F, t1888: F, t1897: F, t1901: F, t1904: F, t1910: F, t1913: F, t1916: F, t2037: F, t2789: F, t2795: F, t2800: F) -> (F, F) {
    let t3152 = 8.0 * t2763 + t1788 + 0.1301229756036208781e0 * t2765 - 0.1143056e0 * t2780 - t1803 - t1806 - t1808 + t1812 + t1816 + t1825 - t1829 - t1833 - t1840 + t1844 + t1851;
    let t3156 = -t1856 + t1858 - 0.10843581300301739842e-1 * t2789 - 2.0 * t2795 - t1874 - t1885 - t1888 - t1897 - t1901 + t1904 + t1910 + t1913 - t1916 + 0.16936279733333333333e-2 * t2800 - t2037;
    (t3152, t3156)
}
