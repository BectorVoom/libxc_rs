//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1360/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1360<F: Float>(t6248: F, t83310: F, t1407: F, t31419: F, t820: F, t27574: F, t31415: F, t28680: F, t111830: F, t111838: F, t112071: F, t123255: F, t123415: F, t123421: F, t127128: F, t127135: F, t127139: F, t127147: F, t127151: F, t127158: F, t127160: F, t19230: F, t231: F, t28552: F, t28584: F, t28603: F, t6035: F, t6045: F, t6249: F, t6251: F, t684: F, t70786: F, t98432: F, t98520: F) -> (F, F, F) {
    let t127163 = t83310 * t6248;
    let t127167 = t1407 * t31419 * t820;
    let t127170 = t27574 * t31415;
    let t127171 = t28680 * t127170;
    let t127173 = 0.11853866860905349795e0 * t28552 * t123255 - 0.10001700163888888889e0 * t98520 * t6035 * t127128 * t684 + 0.22226000364197530866e-1 * t98432 + 0.1611184118048991131e0 * t111830 * t127135 - 0.1611184118048991131e0 * t111838 * t127139 - 0.1611184118048991131e0 * t28603 * t123415 + 0.10741227453659940874e0 * t28603 * t123421 - 0.9667104708293946786e0 * t112071 * t127147 + 0.17780800291358024692e0 * t127151 + 0.10001700163888888889e0 * t6249 * t6045 * t231 * t19230 - 0.10001700163888888889e0 * t127158 - 0.20003400327777777778e0 * t127160 * t28584 + 0.20003400327777777778e0 * t127163 * t6251 + 0.18122740165211489339e1 * t70786 * t127167 - 0.1611184118048991131e0 * t127171;
    (t127167, t127170, t127173)
}
