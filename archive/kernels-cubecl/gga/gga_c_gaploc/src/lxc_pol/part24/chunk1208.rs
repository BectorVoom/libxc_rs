//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1208/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1208<F: Float>(t10632: F, t5227: F, t161: F, t1841: F, t24884: F, t2576: F, t29160: F, t29162: F, t29184: F, t29186: F, t29210: F, t29212: F, t29224: F, t29226: F, t29230: F, t29233: F, t29242: F, t32104: F) -> F {
    let t32106 = F::cast_from(0.51270174867614828558e-2_f64) * t5227 * t10632;
    let t32110 = F::cast_from(0.51270174867614828558e-2_f64) * t1841 * t24884 * t161 * t2576;
    let t32111 = t29160 - t29162 + t29184 + t29186 - t29210 - t29212 - t29224 - t29226 - t29230 + t29233 - t29242 - t32104 + t32106 + t32110;
    t32111
}
