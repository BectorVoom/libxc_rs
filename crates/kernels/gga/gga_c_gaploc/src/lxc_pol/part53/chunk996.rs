//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 996/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk996<F: Float>(t47803: F, t6717: F, t6914: F, t12079: F, t2389: F, t12092: F, t2482: F, t9267: F, t12000: F, t123: F, t883: F, t2487: F, t2488: F) -> (F, F, F, F, F) {
    let t47864 = t6914 * t6717 * t47803;
    let t47866 = t12079 * t2389;
    let t47869 = t9267 * t12092 * t2482;
    let t47877 = t12000 * t123 * t883;
    let t47879 = t2487 * t2488 * t47877;
    (t47864, t47866, t47869, t47877, t47879)
}
