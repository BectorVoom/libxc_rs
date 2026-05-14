//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1067/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1067<F: Float>(t12747: F, t1761: F, t4396: F, t5924: F, t6265: F, t1016: F, t1838: F, t1165: F, t1173: F, t1180: F, t1181: F, t16690: F, t16692: F, t16694: F, t16701: F, t16703: F, t16705: F, t4289: F, t4314: F, t4680: F, t6269: F, t6399: F) -> (F,) {
    let t21583 = t12747 * t1761;
    let t21592 = t4396 * t5924;
    let t21594 = t4396 * t6265;
    let t21596 = t1016 * t1838;
    let t21601 = -0.17149607247227894789e-2 * t16690 - 0.34299214494455789578e-2 * t16692 - 0.34299214494455789578e-2 * t16694 - 0.13605355082800796533e0 * t16701 - 0.90702367218671976884e-1 * t16703 + 0.24009450146119052704e-1 * t16705 + 0.22675591804667994221e-1 * t21583 - 0.17149607247227894789e-2 * t1180 * t4680 * t6399 + 0.68598428988911579156e-2 * t1173 * t1181 * t4289 * t6269 - 0.17149607247227894789e-2 * t21592 + 0.17149607247227894789e-2 * t21594 - 0.12862205435420921092e-2 * t1180 * t1165 * t21596 * t4314;
    (t21601,)
}
