//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1120/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1120<F: Float>(t151: F, t2124: F, t2126: F, t2168: F, t29117: F, t3467: F, t38033: F, t49019: F, t49023: F, t56073: F, t56074: F, t56077: F, t56081: F, t56102: F, t56114: F, t56131: F, t56139: F, t56153: F, t56346: F, t695: F, t696: F, t7129: F, t9961: F) -> (F,) {
    let t56587 = -0.15114211337509259186e-1 * t695 * t696 * t56346 + 0.81136173904695073308e0 * t49019 + 0.81136173904695073308e0 * t49023 + 0.23981215322181357908e2 * t38033 + 0.11719669564011510589e2 * t29117 + 0.62590762726479056551e1 * t3467 * t7129 * t56102 + 0.69545291918310062836e0 * t2124 * t2126 * t56073 + 0.24182738140014814697e0 * t2168 * t56074 + 0.69545291918310062836e0 * t2124 * t2126 * t56114 - 0.34772645959155031419e0 * t2124 * t151 * t56153 - 0.2086358757549301885e1 * t3467 * t2126 * t56081 + 0.417271751509860377e1 * t9961 * t2126 * t56131 + 0.31295381363239528276e1 * t3467 * t151 * t56139 - 0.31295381363239528276e1 * t2124 * t7129 * t56077;
    (t56587,)
}
