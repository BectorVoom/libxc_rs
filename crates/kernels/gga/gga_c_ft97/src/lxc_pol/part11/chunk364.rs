//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 364/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk364<F: Float>(t358: F, t487: F, t363: F, t492: F, t1909: F, t110: F, t1647: F, t447: F, t1822: F, t1827: F, t1843: F, t1848: F, t1855: F, t1859: F, t1863: F, t1868: F, t1873: F, t1878: F, t1883: F, t1887: F, t1888: F, t1890: F, t1893: F, t1897: F, t1901: F, t1906: F, t28: F, t446: F, t89: F) -> (F, F, F, F, F, F) {
    let t1910 = t487 * t358;
    let t1911 = t363 * t492;
    let t1912 = t1910 * t1911;
    let t1913 = t1909 * t1912;
    let t1917 = t447 * t110 * t1647;
    let t1920 = -t446 * t1822 / 3.0 - 2.0 / 3.0 * t446 * t1827 + t89 * t28 * t1843 / 3.0 - 2.0 / 9.0 * t1848 + 2.0 / 3.0 * t446 * t1855 - 2.0 / 9.0 * t446 * t1859 - t446 * t1863 / 9.0 - 2.0 / 27.0 * t446 * t1868 + 2.0 / 3.0 * t446 * t1873 + 2.0 / 3.0 * t446 * t1878 + 2.0 / 27.0 * t1883 + t1887 + 2.0 / 9.0 * t1888 + 2.0 / 9.0 * t1890 - 2.0 / 3.0 * t446 * t1893 - t446 * t1897 / 3.0 + 2.0 / 9.0 * t1901 * t1906 + 2.0 / 9.0 * t1901 * t1913 + 2.0 / 9.0 * t446 * t1917;
    (t1910, t1911, t1912, t1913, t1917, t1920)
}
