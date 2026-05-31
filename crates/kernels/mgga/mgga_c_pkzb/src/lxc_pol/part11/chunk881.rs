//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 881/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk881<F: Float>(t5522: F, t5812: F, t7357: F, t7516: F, t9148: F, t9163: F, t3559: F, t694: F, t1096: F, t1950: F, t248: F, t2796: F, t2816: F, t3565: F, t3592: F, t3605: F, t5897: F, t5903: F, t704: F, t7447: F, t9345: F, t9363: F, t9365: F, t9367: F, t9392: F, t9394: F) -> (F, F, F) {
    let t9515 = -t5812 + F::cast_from(0.22831111111111111111e-1_f64) * t5522 + F::cast_from(0.45662222222222222221e-1_f64) * t7357 - t7516 - F::cast_from(0.17123333333333333333e-1_f64) * t9148 + F::cast_from(0.5137e-1_f64) * t9163;
    let t9518 = t3559 * t694;
    let t9527 = -F::cast_from(0.11696447245269292414e1_f64) * t5903 * t3592 + F::cast_from(0.5848223622634646207e0_f64) * t1950 * t3605 - F::cast_from(0.310907e-1_f64) * t9515 * t248 + t9345 - t9363 + t9365 - t9367 - t9392 - t9394 + F::cast_from(1.0_f64) * t9518 * t704 + F::cast_from(2.0_f64) * t7447 * t1096 + F::cast_from(2.0_f64) * t2796 * t2816 - F::cast_from(2.0_f64) * t5897 * t3565;
    (t9515, t9518, t9527)
}
