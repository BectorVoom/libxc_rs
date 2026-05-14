//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 864/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk864<F: Float>(t41736: F, t46190: F, t46191: F, t46195: F, t46212: F, t46216: F, t46220: F, t46223: F, t46225: F, t46229: F, t46233: F, t46235: F, t46237: F, t47873: F, t47874: F, t47875: F, t47879: F, t47883: F, t47885: F, t47892: F) -> (F,) {
    let t50594 = -t47873 + t47874 + t47875 + t46190 + 0.9585731488480187419e0 * t46191 - 0.21301625529955972042e0 * t46195 + 0.76685851907841499354e0 * t47879 - 0.17041300423964777634e0 * t47883 + 0.76685851907841499354e0 * t47885 - t41736 + 0.11916829983950142223e0 * t47892 + t46212 + t46216 + t46220 - t46223 - t46225 - t46229 - t46233 - 0.89376224879626066674e-1 * t46235 + 0.44688112439813033337e-1 * t46237;
    (t50594,)
}
