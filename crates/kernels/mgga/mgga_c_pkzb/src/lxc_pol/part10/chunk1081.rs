//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1081/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1081<F: Float>(t5522: F, t5812: F, t7357: F, t7516: F, t9148: F, t9163: F, t3559: F, t694: F, t1096: F, t1950: F, t248: F, t2796: F, t2816: F, t3565: F, t3592: F, t3605: F, t5897: F, t5903: F, t704: F, t7447: F, t9345: F, t9363: F, t9365: F, t9367: F, t9392: F, t9394: F) -> (F, F, F) {
    let t9515 = -t5812 + 0.22831111111111111111e-1 * t5522 + 0.45662222222222222221e-1 * t7357 - t7516 - 0.17123333333333333333e-1 * t9148 + 0.5137e-1 * t9163;
    let t9518 = t3559 * t694;
    let t9527 = -0.11696447245269292414e1 * t5903 * t3592 + 0.5848223622634646207e0 * t1950 * t3605 - 0.310907e-1 * t9515 * t248 + t9345 - t9363 + t9365 - t9367 - t9392 - t9394 + 1.0 * t9518 * t704 + 2.0 * t7447 * t1096 + 2.0 * t2796 * t2816 - 2.0 * t5897 * t3565;
    (t9515, t9518, t9527)
}
