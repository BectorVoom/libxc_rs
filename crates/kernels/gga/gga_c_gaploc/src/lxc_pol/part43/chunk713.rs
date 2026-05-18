//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 713/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk713<F: Float>(t12533: F, t12536: F, t12065: F, t895: F, t11986: F, t874: F, t1445: F, t574: F, t13728: F, t597: F, t12054: F, t3377: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13795 = F::new(0.38342925953920749677e0) * t12533;
    let t13796 = F::new(0.38342925953920749677e0) * t12536;
    let t13798 = t895 * t12065;
    let t13800 = t11986 * t874;
    let t13801 = t1445 * t13800;
    let t13802 = t574 * t13801;
    let t13805 = t1445 * t13728;
    let t13806 = t597 * t13805;
    let t13808 = t12054 * t3377;
    (t13795, t13796, t13798, t13800, t13801, t13802, t13805, t13806, t13808)
}
