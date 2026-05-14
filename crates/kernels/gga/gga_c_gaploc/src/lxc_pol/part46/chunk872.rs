//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 872/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk872<F: Float>(t41430: F, t41435: F, t41445: F, t33575: F, t787: F, t10024: F, t24549: F, t7584: F, t9438: F, t13064: F, t825: F, t826: F, t10677: F, t2464: F, t2465: F, t10782: F, t2684: F) -> (F, F, F, F, F, F, F, F) {
    let t44110 = 0.19171462976960374838e1 * t41430;
    let t44111 = 0.42603251059911944084e0 * t41435;
    let t44112 = 0.29792074959875355558e-1 * t41445;
    let t44113 = t787 * t33575;
    let t44114 = t44113 * t10024;
    let t44117 = t7584 * t9438 * t24549;
    let t44118 = 0.15976219147466979032e-1 * t44117;
    let t44120 = t825 * t826 * t13064;
    let t44124 = t825 * t2464 * t2465 * t10677;
    let t44128 = t2684 * t2464 * t2465 * t10782;
    (t44110, t44111, t44112, t44114, t44118, t44120, t44124, t44128)
}
