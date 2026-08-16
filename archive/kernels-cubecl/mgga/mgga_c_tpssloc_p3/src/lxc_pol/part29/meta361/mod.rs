//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1457;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1458;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta361<F: Float>(t9902: F, t2535: F, t4199: F, t1471: F, t32: F, t2659: F, t9910: F, t4095: F, t67: F, t758: F, t9922: F, t118: F, t1474: F, t2375: F, t184: F, t3966: F, t607: F, t4194: F, t12606: F, t185: F, t707: F, t4094: F, t706: F, t708: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13112, t13114, t13117, t13118, t13121, t13122, t13123) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1457::<F>(t9902, t2535, t4199, t1471, t32, t2659, t9910, t4095, t67, t758, t9922, t118, t1474);
        let (t13125, t13129, t13132, t13135) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1458::<F>(t13123, t2375, t184, t3966, t607, t4194, t12606, t185, t707, t4094, t706, t708);
    (t13112, t13114, t13117, t13118, t13121, t13122, t13125, t13129, t13132, t13135)
}
