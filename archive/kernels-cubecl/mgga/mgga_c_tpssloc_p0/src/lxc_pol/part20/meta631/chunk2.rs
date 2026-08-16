//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2298/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2298<F: Float>(t13068: F, t225: F, t13030: F, t10046: F, t10049: F, t10104: F, t10111: F, t10112: F, t13053: F, t13065: F, t13463: F, t1492: F, t1527: F, t1528: F, t259: F, t2720: F, t2743: F, t40852: F, t40875: F, t40890: F, t4147: F, t41554: F, t4268: F, t4301: F, t855: F, t866: F) -> F {
    let t47568 = t13068 * t225;
    let t47585 = t13030 * t225;
    let t47593 = F::cast_from(24.0_f64) * t10111 * t1527 * t40890 * t855 + t10046 * t1492 * t259 - F::cast_from(3.0_f64) * t10049 * t4301 - t10104 * t4268 - F::cast_from(6.0_f64) * t10112 * t4147 - F::cast_from(3.0_f64) * t13053 * t2743 + F::cast_from(6.0_f64) * t13065 * t2720 + F::cast_from(6.0_f64) * t13463 * t2720 - t1528 * t40852 - t1528 * t40875 - F::cast_from(3.0_f64) * t1528 * t41554 - F::cast_from(6.0_f64) * t47568 * t866 - F::cast_from(3.0_f64) * t47585 * t866;
    t47593
}
