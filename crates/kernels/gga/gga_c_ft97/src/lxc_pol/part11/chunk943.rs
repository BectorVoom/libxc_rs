//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 943/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk943<F: Float>(t41831: F, t41835: F, t41839: F, t41844: F, t41846: F, t41852: F, t41855: F, t41859: F, t41863: F, t41867: F, t41870: F, t41873: F, t41877: F, t41882: F, t41886: F, t41891: F, t41895: F, t41898: F, t41901: F, t41905: F, t41909: F, t41915: F, t41918: F, t41922: F, t41925: F, t41927: F, t41932: F, t41935: F, t41938: F, t41942: F) -> (F, F) {
    let t42025 = 8.0 / 9.0 * t41831 - 4.0 / 3.0 * t41835 - 4.0 / 3.0 * t41839 + 2.0 * t41844 + 8.0 / 3.0 * t41846 + 8.0 * t41852 + 16.0 / 9.0 * t41855 + 2.0 / 3.0 * t41859 + 4.0 / 9.0 * t41863 + 8.0 / 3.0 * t41867 - 4.0 * t41870 + 8.0 / 3.0 * t41873 - 4.0 / 9.0 * t41877 - 8.0 / 9.0 * t41882 - 16.0 / 9.0 * t41886;
    let t42042 = 8.0 / 3.0 * t41891 - 2.0 / 9.0 * t41895 + 16.0 / 27.0 * t41898 + 8.0 / 9.0 * t41901 + 8.0 / 3.0 * t41905 + 2.0 / 3.0 * t41909 - 80.0 / 243.0 * t41915 + 4.0 / 27.0 * t41918 - 8.0 / 3.0 * t41922 - 8.0 / 3.0 * t41925 + 16.0 / 27.0 * t41927 + 4.0 / 9.0 * t41932 + 8.0 / 9.0 * t41935 - 8.0 / 27.0 * t41938 + 8.0 / 3.0 * t41942;
    (t42025, t42042)
}
