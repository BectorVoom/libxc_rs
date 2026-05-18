//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 780/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk780<F: Float>(t10473: F, t2482: F, t9263: F, t10469: F, t9267: F, t2476: F, t26922: F, t9438: F, t10268: F, t4391: F, t549: F, t12996: F, t18067: F) -> (F, F, F, F, F) {
    let t41609 = t9263 * t10473 * t2482;
    let t41612 = t9267 * t10469 * t2482;
    let t41615 = t2476 * t9438 * t26922;
    let t41618 = t4391 * t549 * t10268;
    let t41623 = t18067 * t12996;
    (t41609, t41612, t41615, t41618, t41623)
}
