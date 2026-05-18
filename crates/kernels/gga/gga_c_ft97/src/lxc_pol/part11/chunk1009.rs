//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1009/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1009<F: Float>(t1882: F, t9446: F, t2170: F, t8232: F, t9278: F, t9430: F, t2182: F, t12746: F, t13165: F, t13220: F, t1580: F, t1901: F, t1986: F, t2075: F, t2157: F, t2180: F, t2210: F, t2211: F, t2221: F, t2222: F, t3434: F, t3439: F, t3440: F, t379: F, t38930: F, t38960: F, t40772: F, t446: F, t616: F, t9017: F, t9093: F, t9099: F, t9121: F, t9133: F, t9288: F, t9432: F) -> (F, F, F, F, F, F) {
    let t41123 = t1882 * t9446;
    let t41125 = t8232 * t2170;
    let t41127 = t1882 * t9278;
    let t41137 = t1882 * t9430;
    let t41139 = t8232 * t2182;
    let t41196 = -F::new(8.0) * t446 * t9432 * t616 * t9017 - F::new(8.0) / F::new(3.0) * t1901 * t13220 * t9288 * t379 + F::new(4.0) / F::new(3.0) * t1901 * t9099 * t9093 - F::new(4.0) / F::new(3.0) * t1901 * t2210 * t13165 * t1580 * t2180 - F::new(4.0) / F::new(3.0) * t1901 * t9133 * t2222 * t1580 * t1986 + F::new(8.0) / F::new(3.0) * t1901 * t3439 * t12746 * t38930 + F::new(2.0) / F::new(3.0) * t1901 * t2221 * t2222 * t1580 * t2075 + F::new(2.0) / F::new(3.0) * t1901 * t2210 * t2211 * t1580 * t2157 + F::new(8.0) / F::new(3.0) * t1901 * t2210 * t9121 * t40772 + F::new(8.0) / F::new(9.0) * t1901 * t2210 * t3434 * t38960 - F::new(8.0) / F::new(27.0) * t1901 * t3439 * t3440 * t38960;
    (t41123, t41125, t41127, t41137, t41139, t41196)
}
