//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 899/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk899<F: Float>(t8392: F, t9146: F, t599: F, t7943: F, t89: F, t1882: F, t9412: F, t9318: F, t9313: F, t9306: F, t161: F, t38061: F, t597: F, t9114: F, t12982: F, t13212: F, t144: F, t1651: F, t1901: F, t1986: F, t2179: F, t2180: F, t2185: F, t2190: F, t39646: F, t39660: F, t446: F, t558: F, t574: F, t9099: F, t9117: F, t9123: F, t9144: F, t9349: F, t9354: F, t9419: F, t9439: F, t9440: F) -> (F, F, F, F, F, F) {
    let t41047 = t8392 * t9146;
    let t41050 = t89 * t7943 * t599;
    let t41064 = t1882 * t9412;
    let t41074 = t1882 * t9318;
    let t41076 = t1882 * t9313;
    let t41084 = t1882 * t9306;
    let t41093 = 280.0 / 243.0 * t89 * t38061 * t161;
    let t41107 = t9114 * t597;
    let t41117 = 8.0 * t446 * t574 * t9439 * t9440 * t558 + 8.0 / 3.0 * t41084 + 8.0 * t446 * t2185 * t2179 * t1986 * t2180 + t41093 - 4.0 / 3.0 * t1901 * t9144 * t1651 * t2190 + 8.0 / 9.0 * t1901 * t13212 * t39660 + 8.0 / 3.0 * t446 * t144 * t39646 + 8.0 / 9.0 * t1901 * t12982 * t9123 + 8.0 / 9.0 * t1901 * t41107 * t9117 + 4.0 / 3.0 * t1901 * t9419 * t9349 + 4.0 / 3.0 * t1901 * t9099 * t9354;
    (t41047, t41050, t41064, t41074, t41076, t41117)
}
