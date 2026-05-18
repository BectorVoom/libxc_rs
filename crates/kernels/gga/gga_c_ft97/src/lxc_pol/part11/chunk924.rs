//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 924/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk924<F: Float>(t1859: F, t8232: F, t1882: F, t8579: F, t1786: F, t1852: F, t11863: F, t1643: F, t1651: F, t1853: F, t1866: F, t1871: F, t1901: F, t1904: F, t1922: F, t358: F, t37298: F, t39093: F, t39095: F, t39097: F, t446: F, t447: F, t499: F, t7973: F, t8544: F) -> F {
    let t39099 = t8232 * t1859;
    let t39101 = t1882 * t8579;
    let t39107 = t1786 * t1852;
    let t39116 = -F::new(4.0) / F::new(9.0) * t446 * t447 * t499 * t7973 - F::new(4.0) / F::new(9.0) * t446 * t1866 * t1922 * t1643 - F::new(2.0) / F::new(3.0) * t446 * t447 * t1922 * t1651 + F::new(112.0) / F::new(243.0) * t39093 + F::new(8.0) / F::new(9.0) * t39095 + F::new(16.0) / F::new(9.0) * t39097 - F::new(16.0) / F::new(27.0) * t39099 + F::new(40.0) / F::new(243.0) * t39101 + F::new(8.0) * t446 * t1871 * t499 * t8544 + F::new(8.0) / F::new(3.0) * t1901 * t39107 * t1853 * t358 * t1904 - F::new(8.0) / F::new(3.0) * t1901 * t11863 * t37298;
    t39116
}
