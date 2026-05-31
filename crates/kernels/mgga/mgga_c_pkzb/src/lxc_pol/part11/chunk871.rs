//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 871/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk871<F: Float>(t730: F, t9359: F, t2746: F, t2783: F, t3525: F, t5734: F, t1850: F, t3551: F, t5522: F, t5783: F, t7357: F, t7420: F, t9138: F, t9140: F, t9143: F, t9148: F, t9163: F, t9165: F, t9172: F, t9174: F) -> (F, F, F, F, F) {
    let t9361 = F::cast_from(0.34631718211362927518e2_f64) * t730 * t9359;
    let t9363 = F::cast_from(2.0_f64) * t2746 * t2783;
    let t9365 = F::cast_from(2.0_f64) * t5734 * t3525;
    let t9367 = F::cast_from(1.0_f64) * t1850 * t3551;
    let t9378 = F::cast_from(0.142419375e1_f64) * t9138 - F::cast_from(0.1898925e1_f64) * t9140 - F::cast_from(0.9494625e0_f64) * t9143 + F::cast_from(0.1898925e1_f64) * t9165 - t5783 + F::cast_from(0.39862222222222222223e0_f64) * t5522 + F::cast_from(0.79724444444444444445e0_f64) * t7357 - t7420 - F::cast_from(0.29896666666666666667e0_f64) * t9148 + F::cast_from(0.8969e0_f64) * t9163 - F::cast_from(0.76790625e-1_f64) * t9172 + F::cast_from(0.3071625e0_f64) * t9174;
    (t9361, t9363, t9365, t9367, t9378)
}
