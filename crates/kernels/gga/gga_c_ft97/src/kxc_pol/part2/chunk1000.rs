//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 1000/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk1000<F: Float>(t1882: F, t4178: F, t4183: F, t1255: F, t2413: F, t835: F, t2405: F, t2857: F, t10447: F, t4151: F, t14116: F, t4140: F) -> (F, F, F, F, F, F) {
    let t15500 = F::new(2.0) / F::new(9.0) * t1882 * t4178;
    let t15502 = F::new(4.0) / F::new(9.0) * t1882 * t4183;
    let t15504 = t835 * t1255 * t2413;
    let t15508 = t2857 * t1255 * t2405;
    let t15511 = t10447 * t4151;
    let t15514 = t4140 * t14116;
    (t15500, t15502, t15504, t15508, t15511, t15514)
}
