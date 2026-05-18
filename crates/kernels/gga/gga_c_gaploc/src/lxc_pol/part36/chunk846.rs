//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 846/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk846<F: Float>(t40202: F, t2375: F, t34558: F, t2478: F, t3358: F, t6576: F, t3177: F, t8272: F, t9267: F, t40208: F, t12953: F, t4781: F) -> (F, F, F, F, F, F) {
    let t41893 = F::new(0.46011511144704899612e1) * t40202;
    let t41897 = t34558 * t2375;
    let t41900 = t6576 * t3358 * t2478;
    let t41903 = t9267 * t8272 * t3177;
    let t41904 = F::new(0.19171462976960374838e1) * t41903;
    let t41905 = F::new(0.10352590007558602413e2) * t40208;
    let t41906 = t4781 * t12953;
    (t41893, t41897, t41900, t41904, t41905, t41906)
}
