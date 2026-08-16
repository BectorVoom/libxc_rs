//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1282/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1282<F: Float>(t25901: F, t94921: F, t10115: F, t2024: F, t1445: F, t25921: F, t26079: F, t26081: F, t4003: F, t7279: F, t7295: F, t94628: F, t94895: F, t94898: F, t94902: F, t94904: F, t94906: F, t94909: F, t94911: F, t94914: F, t94917: F, t94919: F, t9652: F) -> F {
    let t94922 = t94921 * t25901;
    let t94931 = F::cast_from(0.11044544084478153697e-3_f64) * t10115 * t2024;
    let t94934 = F::cast_from(0.21684070470512998656e-1_f64) * t94895 + F::cast_from(0.16463622957338778996e-1_f64) * t94898 + F::cast_from(0.58544643236296698113e-1_f64) * t94902 + F::cast_from(0.43368140941025997312e-1_f64) * t94904 - F::cast_from(0.19756347548806534796e1_f64) * t94906 * t1445 + F::cast_from(0.77108554593144223218e-1_f64) * t94909 + F::cast_from(0.38554277296572111609e-1_f64) * t94911 + F::cast_from(0.51405703062096148814e-2_f64) * t94914 + t94917 - F::cast_from(0.72280234901709995519e-3_f64) * t94919 - F::cast_from(0.43368140941025997312e-1_f64) * t94922 - F::cast_from(0.26020884564615598386e1_f64) * t25921 * t26081 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t26079 * t94628 * t4003 - t94931 + F::cast_from(0.39512695097613069591e1_f64) * t7279 * t9652;
    t94934
}
