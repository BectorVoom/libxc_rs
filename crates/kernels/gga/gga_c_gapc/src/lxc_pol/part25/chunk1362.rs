//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1362/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1362<F: Float>(t3537: F, t8598: F, t12291: F, t7056: F, t10091: F, t31783: F, t12058: F, t4908: F, t10099: F, t10791: F, t12285: F, t7063: F, t972: F) -> (F, F, F, F, F, F) {
    let t36455 = F::new(2.0) * t8598 * t3537;
    let t36457 = F::new(4.0) * t7056 * t12291;
    let t36460 = F::new(6.0) * t31783 * t10091;
    let t36462 = F::new(4.0) * t4908 * t12058;
    let t36467 = F::new(4.0) * t10099 * t10791;
    let t36470 = F::new(12.0) * t7063 * t12285 * t972;
    (t36455, t36457, t36460, t36462, t36467, t36470)
}
