//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1198/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1198<F: Float>(t2453: F, t25949: F, t25946: F, t25939: F, t40270: F, t10073: F, t25920: F, t25938: F, t25898: F, t94889: F, t25901: F, t10115: F, t2024: F, t1445: F, t25921: F, t26079: F, t26081: F, t4003: F, t7279: F, t7295: F, t94628: F, t94895: F, t94898: F, t94902: F, t94904: F, t94906: F, t94909: F, t94911: F, t9652: F) -> (F,) {
    let t94913 = t2453 * t25949;
    let t94914 = t94913 * t25946;
    let t94917 = 0.96373646535613327356e-3 * t40270 * t25939;
    let t94919 = t10073 * t25920 * t25938;
    let t94921 = t94889 * t25898;
    let t94922 = t94921 * t25901;
    let t94931 = 0.11044544084478153697e-3 * t10115 * t2024;
    let t94934 = 0.21684070470512998656e-1 * t94895 + 0.16463622957338778996e-1 * t94898 + 0.58544643236296698113e-1 * t94902 + 0.43368140941025997312e-1 * t94904 - 0.19756347548806534796e1 * t94906 * t1445 + 0.77108554593144223218e-1 * t94909 + 0.38554277296572111609e-1 * t94911 + 0.51405703062096148814e-2 * t94914 + t94917 - 0.72280234901709995519e-3 * t94919 - 0.43368140941025997312e-1 * t94922 - 0.26020884564615598386e1 * t25921 * t26081 - 0.26020884564615598386e1 * t7295 * t26079 * t94628 * t4003 - t94931 + 0.39512695097613069591e1 * t7279 * t9652;
    (t94934,)
}
