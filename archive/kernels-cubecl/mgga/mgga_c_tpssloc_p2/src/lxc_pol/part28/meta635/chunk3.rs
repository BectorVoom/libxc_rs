//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2014/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2014<F: Float>(t90844: F, t90859: F, t90864: F, t90866: F, t90868: F, t1332: F, t1336: F, t16047: F, t16048: F, t16055: F, t24117: F, t24131: F, t27074: F, t27075: F, t27097: F, t27105: F, t3793: F, t3856: F, t5234: F, t5334: F, t81022: F, t90848: F, t90852: F, t90856: F, t90873: F) -> F {
    let t93524 = F::cast_from(0.3289868133696452873e-1_f64) * t90844;
    let t93528 = F::cast_from(0.16449340668482264365e-1_f64) * t90859;
    let t93529 = F::cast_from(0.16449340668482264365e-1_f64) * t90864;
    let t93537 = F::cast_from(0.76763589786250567036e-1_f64) * t90866;
    let t93538 = F::cast_from(0.12793931631041761173e0_f64) * t90868;
    let t93546 = -F::cast_from(0.16449340668482264365e-1_f64) * t81022 + F::cast_from(4.0_f64) * t16055 * t27075 - t93524 + F::cast_from(0.6579736267392905746e-1_f64) * t90848 - F::cast_from(0.16449340668482264365e-1_f64) * t90852 + F::cast_from(0.9869604401089358619e-1_f64) * t90856 + t93528 + t93529 - F::cast_from(6.0_f64) * t16047 * t27074 * t16048 + F::cast_from(6.0_f64) * t5334 * t27074 * t3793 - t5234 * t24131 - t93537 + t93538 + F::cast_from(2.0_f64) * t1332 * t27105 - t1336 * t27097 * t3856 - F::cast_from(2.0_f64) * t5234 * t24117 - F::cast_from(0.16449340668482264365e-1_f64) * t90873;
    t93546
}
