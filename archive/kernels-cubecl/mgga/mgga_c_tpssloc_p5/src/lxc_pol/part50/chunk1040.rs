//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1040/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1040<F: Float>(t225: F, t387: F, t6768: F, t345: F, t1065: F, t8396: F, t10165: F, t8391: F, t990: F, t6726: F, t8384: F, t1948: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t30800 = t6768 * t225 * t387;
    let t30801 = t345 * t30800;
    let t30804 = t8396 * t1065;
    let t30805 = t10165 * t30804;
    let t30808 = t990 * t8391;
    let t30813 = F::cast_from(0.40372756094140390856e-3_f64) * t6726 * t8384;
    let t30816 = t1948 * sigma0;
    (t30800, t30801, t30805, t30808, t30813, t30816)
}
