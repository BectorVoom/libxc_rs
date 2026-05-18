//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1222/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1222<F: Float>(t11220: F, t1282: F, t1291: F, t14664: F, t14672: F, t14674: F, t14676: F, t14679: F, t15093: F, t15109: F, t15690: F, t15692: F, t15788: F, t1872: F, t3670: F, t437: F) -> F {
    let t15790 = -t11220 * t1872 - t1282 * t15788 - F::new(2.0) * t1291 * t15109 + t15690 * t437 + F::new(2.0) * t15692 * t3670 - t14664 + t14672 - t14674 + t14676 + t14679 + t15093;
    t15790
}
