//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1380/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1380<F: Float>(t45936: F, t584: F, t596: F, t20: F, t2237: F, t12: F, t14: F, t27: F, t10285: F, t2231: F, t10293: F, t592: F) -> (F, F, F, F, F, F, F) {
    let t45937 = F::cast_from(1440.0_f64) * t45936;
    let t45938 = t584 * t596;
    let t45939 = F::cast_from(1920.0_f64) * t45938;
    let t45941 = F::cast_from(840.0_f64) * t20 * t2237;
    let t45944 = F::cast_from(360.0_f64) * t12 * t14 * t27;
    let t45945 = t10285 * t596;
    let t45946 = F::cast_from(2880.0_f64) * t45945;
    let t45947 = t2231 * t2237;
    let t45948 = F::cast_from(7560.0_f64) * t45947;
    let t45949 = t592 * t10293;
    (t45937, t45939, t45941, t45944, t45946, t45948, t45949)
}
