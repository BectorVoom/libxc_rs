//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 690/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk690<F: Float>(t1645: F, t7696: F, t22980: F, t2615: F, t9438: F, t22984: F, t7584: F, t12692: F, t2013: F, t10007: F, t2530: F, t825: F, t12705: F, t7416: F, t10012: F, t2684: F) -> (F, F, F, F, F, F, F) {
    let t41105 = t1645 * t7696;
    let t41231 = t2615 * t9438 * t22980;
    let t41244 = t7584 * t9438 * t22984;
    let t41295 = t2013 * t12692;
    let t41299 = t825 * t9438 * t10007 * t2530;
    let t41312 = t7416 * t12705;
    let t41316 = t2684 * t9438 * t10012 * t2530;
    (t41105, t41231, t41244, t41295, t41299, t41312, t41316)
}
