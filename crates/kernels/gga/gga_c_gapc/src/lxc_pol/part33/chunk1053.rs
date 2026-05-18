//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1053/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1053<F: Float>(t3406: F, t8133: F, t2579: F, t3412: F, t1615: F, t2962: F, t11295: F, t12007: F, t11282: F, t11285: F, t11610: F, t11289: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30324 = t3406 * t8133;
    let t30325 = t2579 * t3412 * t30324;
    let t30472 = t2962 * t1615;
    let t33091 = F::new(8.0) * t11295;
    let t33093 = F::new(2.0) * t12007;
    let t33094 = F::new(2.0) * t11282;
    let t33095 = F::new(8.0) * t11285;
    let t33096 = F::new(2.0) * t11610;
    let t33097 = F::new(4.0) * t11289;
    (t30324, t30325, t30472, t33091, t33093, t33094, t33095, t33096, t33097)
}
