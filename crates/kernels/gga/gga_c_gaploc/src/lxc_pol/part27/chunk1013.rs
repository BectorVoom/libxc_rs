//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1013/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1013<F: Float>(t2537: F, t7064: F, t7177: F, t1842: F, t21491: F, t883: F, t5538: F, t7305: F, t23296: F, t2558: F, t9647: F, t21476: F, t7313: F, t22238: F, t7301: F, t1841: F, t9649: F) -> (F, F, F, F, F, F, F, F) {
    let t29242 = 0.64087718584518535698e-3 * t7064 * t2537 * t7177;
    let t29273 = 0.3845263115071112142e-2 * t7064 * t1842 * t883 * t21491;
    let t29277 = t5538 * t883;
    let t29280 = 0.2563508743380741428e-2 * t7064 * t29277 * t7305;
    let t29304 = 0.1281754371690370714e-2 * t9647 * t23296 * t2558;
    let t29310 = 0.1281754371690370714e-2 * t21476 * t2537 * t7313;
    let t29324 = 0.4486140300916297499e-2 * t9647 * t22238 * t7301;
    let t29349 = 0.51270174867614828559e-2 * t1841 * t9649;
    (t29242, t29273, t29277, t29280, t29304, t29310, t29324, t29349)
}
