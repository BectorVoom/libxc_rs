//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1288/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1288<F: Float>(t31837: F, t3236: F, t3132: F, t31851: F, t1035: F, t12663: F, t1042: F, t31857: F, t12699: F, t12712: F, t167: F, t12810: F, t2689: F, t116: F, t12681: F, t210: F) -> (F, F, F, F, F, F, F) {
    let t110859 = t3236 * t31837;
    let t110861 = t3132 * t31851;
    let t110863 = t1035 * t12663;
    let t110865 = t1042 * t31857;
    let t110868 = t12712 * t167 * t12699;
    let t110870 = t12810 * t2689;
    let t110873 = t210 * t116 * t12681;
    (t110859, t110861, t110863, t110865, t110868, t110870, t110873)
}
