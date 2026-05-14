//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 956/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk956<F: Float>(t255: F, t42163: F, t10081: F, t10093: F, t14080: F, t14081: F, t1901: F, t242: F, t2599: F, t2600: F, t2609: F, t3891: F, t3892: F, t41421: F, t42374: F, t42376: F, t42385: F, t42392: F, t42394: F, t42399: F, t42404: F, t446: F, t713: F, t8608: F, t9787: F) -> (F,) {
    let t42409 = t42163 * t255;
    let t42414 = 4.0 / 3.0 * t1901 * t9787 * t10093 + 8.0 / 9.0 * t42374 - 8.0 / 3.0 * t1901 * t42376 * t10081 + 4.0 / 9.0 * t1901 * t2599 * t2600 * t8608 * t713 + 4.0 / 3.0 * t1901 * t42385 * t2609 - 4.0 / 3.0 * t446 * t242 * t41421 + 8.0 / 9.0 * t42392 + 8.0 / 3.0 * t1901 * t2599 * t3892 * t42394 - 8.0 / 27.0 * t1901 * t3891 * t3892 * t42399 - 20.0 / 27.0 * t1901 * t14080 * t14081 * t42404 + 40.0 / 81.0 * t1901 * t42409 * t14081 * t42394;
    (t42414,)
}
