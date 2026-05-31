//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 871/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk871<F: Float>(t1628: F, t3060: F, t1589: F, t2963: F, t7088: F, t7090: F, t7093: F, t7096: F, t7098: F, t7100: F, t1020: F, t2919: F, t471: F, t64: F, t90: F) -> (F, F, F) {
    let t8693 = t1628 * t3060;
    let t8696 = t1589 * t2963;
    let t8710 = F::cast_from(189.0_f64) / F::cast_from(256.0_f64) * t7088 - F::cast_from(483.0_f64) / F::cast_from(8192.0_f64) * t7090 + F::cast_from(147.0_f64) / F::cast_from(524288.0_f64) * t7093 - F::cast_from(49.0_f64) / F::cast_from(524288.0_f64) * t7096 + F::cast_from(161.0_f64) / F::cast_from(8192.0_f64) * t7098 - F::cast_from(63.0_f64) / F::cast_from(256.0_f64) * t7100;
    let t8720 = t8710 * t471 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2919 * t64 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1020 * t90 + F::cast_from(63.0_f64) / F::cast_from(256.0_f64) * t7088 - F::cast_from(49.0_f64) / F::cast_from(8192.0_f64) * t7090 + F::cast_from(49.0_f64) / F::cast_from(24576.0_f64) * t7098 - F::cast_from(21.0_f64) / F::cast_from(256.0_f64) * t7100;
    (t8693, t8696, t8720)
}
