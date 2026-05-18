//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1119/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1119<F: Float>(t21476: F, t2537: F, t7313: F, t22238: F, t7301: F, t9647: F, t1841: F, t9649: F, t23021: F, t2558: F, t9652: F, t1843: F, t22045: F) -> (F, F, F, F, F, F) {
    let t29310 = F::new(0.1281754371690370714e-2) * t21476 * t2537 * t7313;
    let t29324 = F::new(0.4486140300916297499e-2) * t9647 * t22238 * t7301;
    let t29349 = F::new(0.51270174867614828559e-2) * t1841 * t9649;
    let t29354 = F::new(0.64087718584518535698e-3) * t9647 * t23021 * t2558;
    let t29434 = F::new(0.34180116578409885706e-2) * t1841 * t9652;
    let t29437 = F::new(0.1281754371690370714e-2) * t21476 * t1843 * t22045;
    (t29310, t29324, t29349, t29354, t29434, t29437)
}
