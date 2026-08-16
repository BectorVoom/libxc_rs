//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 364/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk364(t358: f64, t487: f64, t363: f64, t492: f64, t1909: f64, t110: f64, t1647: f64, t447: f64, t1822: f64, t1827: f64, t1843: f64, t1848: f64, t1855: f64, t1859: f64, t1863: f64, t1868: f64, t1873: f64, t1878: f64, t1883: f64, t1887: f64, t1888: f64, t1890: f64, t1893: f64, t1897: f64, t1901: f64, t1906: f64, t28: f64, t446: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1910 = t487 * t358;
    let t1911 = t363 * t492;
    let t1912 = t1910 * t1911;
    let t1913 = t1909 * t1912;
    let t1917 = t447 * t110 * t1647;
    let t1920 = -t446 * t1822 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t1827 + t89 * t28 * t1843 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t1848 + 2.0_f64 / 3.0_f64 * t446 * t1855 - 2.0_f64 / 9.0_f64 * t446 * t1859 - t446 * t1863 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t1868 + 2.0_f64 / 3.0_f64 * t446 * t1873 + 2.0_f64 / 3.0_f64 * t446 * t1878 + 2.0_f64 / 27.0_f64 * t1883 + t1887 + 2.0_f64 / 9.0_f64 * t1888 + 2.0_f64 / 9.0_f64 * t1890 - 2.0_f64 / 3.0_f64 * t446 * t1893 - t446 * t1897 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t1906 + 2.0_f64 / 9.0_f64 * t1901 * t1913 + 2.0_f64 / 9.0_f64 * t446 * t1917;
    (t1910, t1911, t1912, t1913, t1917, t1920)
}
