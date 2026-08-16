//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1207/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1207<F: Float>(t1516: F, t8343: F, t1527: F, t30633: F, t23270: F, t1888: F, t6571: F, t7537: F, t6553: F, t1880: F, t25224: F, t8335: F) -> (F, F, F, F, F, F, F, F) {
    let t32847 = t8343 * t1516;
    let t32862 = t30633 * t1527;
    let t32863 = t23270 * t32862;
    let t32865 = F::cast_from(0.3289868133696452873e-1_f64) * t1888 * t32863;
    let t32866 = t6571 * t7537;
    let t32867 = t6553 * t32866;
    let t32869 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t32867;
    let t32875 = t25224 * t8335;
    (t32847, t32862, t32863, t32865, t32866, t32867, t32869, t32875)
}
