//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1342/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1342(t25901: f64, t94921: f64, t10115: f64, t2024: f64, t1445: f64, t25921: f64, t26079: f64, t26081: f64, t4003: f64, t7279: f64, t7295: f64, t94628: f64, t94895: f64, t94898: f64, t94902: f64, t94904: f64, t94906: f64, t94909: f64, t94911: f64, t94914: f64, t94917: f64, t94919: f64, t9652: f64) -> f64 {
    let t94922 = t94921 * t25901;
    let t94931 = 0.11044544084478153697e-3_f64 * t10115 * t2024;
    let t94934 = 0.21684070470512998656e-1_f64 * t94895 + 0.16463622957338778996e-1_f64 * t94898 + 0.58544643236296698113e-1_f64 * t94902 + 0.43368140941025997312e-1_f64 * t94904 - 0.19756347548806534796e1_f64 * t94906 * t1445 + 0.77108554593144223218e-1_f64 * t94909 + 0.38554277296572111609e-1_f64 * t94911 + 0.51405703062096148814e-2_f64 * t94914 + t94917 - 0.72280234901709995519e-3_f64 * t94919 - 0.43368140941025997312e-1_f64 * t94922 - 0.26020884564615598386e1_f64 * t25921 * t26081 - 0.26020884564615598386e1_f64 * t7295 * t26079 * t94628 * t4003 - t94931 + 0.39512695097613069591e1_f64 * t7279 * t9652;
    t94934
}
