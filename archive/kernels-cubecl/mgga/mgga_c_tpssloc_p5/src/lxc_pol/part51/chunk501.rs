//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 501/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk501<F: Float>(t2843: F, t2764: F, t919: F, t923: F, t307: F, t922: F, t302: F, t2822: F, t310: F, t938: F, t942: F, t320: F, t941: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2844 = F::cast_from(1.0_f64) / t2843;
    let t2848 = F::cast_from(0.22831111111111111111e-1_f64) * t2764;
    let t2856 = t919 * t923;
    let t2859 = t922 * t307;
    let t2860 = F::cast_from(1.0_f64) / t2859;
    let t2861 = t302 * t2860;
    let t2868 = F::cast_from(0.68863333333333333333e0_f64) * t2764;
    let t2875 = F::cast_from(0.17365833333333333333e0_f64) * t2822;
    let t2884 = t922 * t922;
    let t2885 = F::cast_from(1.0_f64) / t2884;
    let t2886 = t302 * t2885;
    let t2887 = t310 * t310;
    let t2888 = F::cast_from(1.0_f64) / t2887;
    let t2892 = F::cast_from(0.12361111111111111111e-1_f64) * t2764;
    let t2900 = t938 * t942;
    let t2903 = t941 * t320;
    (t2844, t2848, t2856, t2861, t2868, t2875, t2886, t2888, t2892, t2900, t2903)
}
