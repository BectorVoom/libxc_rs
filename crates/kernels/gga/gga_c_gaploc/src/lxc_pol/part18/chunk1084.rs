//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1084/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1084<F: Float>(t2358: F, t27232: F, t10281: F, t501: F, t605: F, t10629: F, t5227: F, t10632: F, t161: F, t1841: F, t24884: F, t2576: F, t29160: F, t29162: F, t29184: F, t29186: F, t29210: F, t29212: F, t29224: F, t29226: F, t29230: F, t29233: F, t29242: F) -> (F, F, F) {
    let t32099 = 4.0 * t27232 * t2358;
    let t32100 = t10281 * t501;
    let t32102 = 2.0 * t32100 * t605;
    let t32104 = 0.34180116578409885704e-2 * t5227 * t10629;
    let t32106 = 0.51270174867614828558e-2 * t5227 * t10632;
    let t32110 = 0.51270174867614828558e-2 * t1841 * t24884 * t161 * t2576;
    let t32111 = t29160 - t29162 + t29184 + t29186 - t29210 - t29212 - t29224 - t29226 - t29230 + t29233 - t29242 - t32104 + t32106 + t32110;
    (t32099, t32102, t32111)
}
