//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1457/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1457<F: Float>(t120874: F, t123111: F, t1774: F, t2036: F, t2096: F, t2314: F, t23938: F, t26977: F, t27147: F, t27188: F, t27219: F, t27858: F, t27879: F, t32349: F, t32365: F, t34170: F, t5107: F, t7042: F, t7266: F, t7271: F, t7458: F, t7989: F, t8829: F) -> F {
    let t124708 = t123111 * t2096 - t1774 * t32349 - t2036 * t27858 - F::cast_from(2.0_f64) * t2314 * t34170 - F::cast_from(2.0_f64) * t23938 * t7989 - F::cast_from(2.0_f64) * t26977 * t7989 - F::cast_from(2.0_f64) * t27147 * t7266 - F::cast_from(2.0_f64) * t27188 * t7271 - F::cast_from(2.0_f64) * t27219 * t7266 - F::cast_from(2.0_f64) * t27879 * t7042 - F::cast_from(2.0_f64) * t32365 * t7458 - t5107 * t8829 + t120874;
    t124708
}
