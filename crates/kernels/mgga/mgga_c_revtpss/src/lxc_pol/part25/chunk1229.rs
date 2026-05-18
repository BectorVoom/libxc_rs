//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1229/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1229<F: Float>(t7049: F, t786: F, t867: F, t2467: F, t2772: F, t689: F, t7014: F, t25338: F, t887: F, t10977: F, t1949: F, t231: F, t25317: F, t25322: F, t25325: F, t25383: F, t25391: F, t25395: F, t25407: F, t25419: F, t27357: F, t2829: F, t7070: F, t7071: F, t7076: F, t7083: F, t886: F, t92884: F, t92891: F, t92895: F, t92901: F, t92905: F, t92907: F, t92917: F) -> F {
    let t92921 = t786 * t7049 * t867;
    let t92922 = t92921 * t2467;
    let t92925 = t689 * t7014 * t2772;
    let t92930 = t689 * t25338 * t887;
    let t92932 = -F::new(0.78062653693846795158e1) * t7070 * t25317 * t25325 * t886 - F::new(0.13010442282307799193e1) * t25407 * t7083 + F::new(0.52041769129231196772e1) * t25391 * t27357 * t92884 - F::new(0.38554277296572111609e-1) * t92891 + F::new(0.51405703062096148814e-2) * t92895 - F::new(0.19756347548806534796e1) * t25322 * t2829 - F::new(0.16463622957338778996e-1) * t92901 + F::new(0.14456046980341999104e-2) * t92905 + F::new(0.4336814094102599731e0) * t7070 * t7076 * t92907 * t231 + F::new(0.8673628188205199462e0) * t7070 * t7071 * t1949 * t10977 - F::new(0.52041769129231196772e1) * t92917 * t25395 - F::new(0.58544643236296698113e-1) * t92922 - F::new(0.32927245914677557992e-1) * t92925 - F::new(0.26020884564615598386e1) * t25383 * t25419 + F::new(0.32927245914677557992e-1) * t92930;
    t92932
}
