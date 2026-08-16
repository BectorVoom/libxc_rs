//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 766/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk766<F: Float>(t187: F, t3850: F, t2608: F, t520: F, t512: F, t189: F, t19: F, t27: F, t521: F, t14: F, t22: F, t583: F, t588: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3852 = F::cast_from(0.19751673498613801407e-1_f64) * t3850 * t187;
    let t3853 = t520 * t2608;
    let t3854 = t512 * t3853;
    let t3855 = t3850 * t189;
    let t3856 = t512 * t3855;
    let t3857 = t19 * t27;
    let t3859 = F::cast_from(20.0_f64) * t3857 * t521;
    let t3860 = t14 * t22;
    let t3862 = F::cast_from(12.0_f64) * t3860 * t521;
    let t3863 = t583 * t588;
    (t3852, t3853, t3854, t3855, t3856, t3857, t3859, t3860, t3862, t3863)
}
