//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1331/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1331<F: Float>(t11000: F, t783: F, t2714: F, t8634: F, t2718: F, t24817: F, t955: F, t14626: F, t2087: F, t3503: F, t1029: F, t7419: F, t9796: F) -> (F, F, F, F, F, F) {
    let t33778 = t11000 * t783;
    let t33786 = F::cast_from(0.71500979903700853338e0_f64) * t2714 * t8634;
    let t33788 = F::cast_from(0.71500979903700853338e0_f64) * t2718 * t8634;
    let t33790 = F::cast_from(0.35750489951850426669e0_f64) * t955 * t24817;
    let t33799 = F::cast_from(0.30674340763136599741e1_f64) * t2087 * t14626 * t3503;
    let t33813 = t9796 * t1029 * t7419;
    (t33778, t33786, t33788, t33790, t33799, t33813)
}
