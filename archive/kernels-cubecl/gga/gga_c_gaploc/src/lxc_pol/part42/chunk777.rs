//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 777/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk777<F: Float>(t12704: F, t2464: F, t2684: F, t1645: F, t7696: F, t22980: F, t2615: F, t9438: F, t22984: F, t7584: F, t12692: F, t2013: F) -> (F, F, F, F, F) {
    let t41071 = t2684 * t2464 * t12704;
    let t41105 = t1645 * t7696;
    let t41231 = t2615 * t9438 * t22980;
    let t41244 = t7584 * t9438 * t22984;
    let t41295 = t2013 * t12692;
    (t41071, t41105, t41231, t41244, t41295)
}
