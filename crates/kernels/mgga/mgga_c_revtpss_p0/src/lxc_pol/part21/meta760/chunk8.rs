//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2692/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2692<F: Float>(t2782: F, t4077: F, t47794: F, t556: F, t1426: F, t5711: F, t786: F, t3917: F, t1424: F, t14269: F, t14299: F, t213: F, t225: F, t4071: F, t4132: F, t47568: F, t47570: F, t47574: F, t47580: F, t47591: F, t49161: F, t49468: F, t49472: F, t49474: F, t49477: F, t49480: F, t561: F, t5774: F, t9657: F) -> F {
    let t49497 = t2782 * t556 * t47794 * t4077;
    let t49503 = t786 * t5711 * t1426;
    let t49504 = t49503 * t3917;
    let t49506 = -F::cast_from(0.17073386770573548589e-1_f64) * t49468 + F::cast_from(0.58544643236296698113e-1_f64) * t49472 - F::cast_from(0.11044544084478153697e-3_f64) * t49474 - t49477 - F::cast_from(0.19514881078765566037e-2_f64) * t49480 - F::cast_from(0.19756347548806534796e1_f64) * t4071 * t14269 + F::cast_from(0.33133632253434461091e-3_f64) * t47568 - F::cast_from(0.11853808529283920877e2_f64) * t1424 * t9657 * t5774 * t4077 - F::cast_from(0.13878983423218070567e-1_f64) * t47570 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t49161 * t225 * t561 - F::cast_from(0.58911598146606471822e-3_f64) * t47574 + F::cast_from(0.98781737744032673976e-1_f64) * t49497 - F::cast_from(0.78059524315062264151e-1_f64) * t47580 - F::cast_from(0.19756347548806534796e1_f64) * t14299 * t4132 - F::cast_from(0.58544643236296698113e-1_f64) * t49504 - t47591;
    t49506
}
