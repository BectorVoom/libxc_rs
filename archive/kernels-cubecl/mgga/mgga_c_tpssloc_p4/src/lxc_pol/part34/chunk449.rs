//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 449/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk449<F: Float>(t275: F, t2841: F, t290: F, t2764: F, t307: F, t922: F, t302: F, t2822: F, t310: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2842 = t275 * t2841;
    let t2843 = t290 * t290;
    let t2844 = F::cast_from(1.0_f64) / t2843;
    let t2848 = F::cast_from(0.22831111111111111111e-1_f64) * t2764;
    let t2859 = t922 * t307;
    let t2860 = F::cast_from(1.0_f64) / t2859;
    let t2861 = t302 * t2860;
    let t2868 = F::cast_from(0.68863333333333333333e0_f64) * t2764;
    let t2875 = F::cast_from(0.17365833333333333333e0_f64) * t2822;
    let t2884 = t922 * t922;
    let t2885 = F::cast_from(1.0_f64) / t2884;
    let t2886 = t302 * t2885;
    let t2887 = t310 * t310;
    (t2842, t2843, t2844, t2848, t2860, t2861, t2868, t2875, t2884, t2885, t2886, t2887)
}
