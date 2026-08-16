//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1456;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1457;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1458;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta383(t120: f64, t5584: f64, t16816: f64, t4180: f64, t4182: f64, t5593: f64, t9638: f64, t5527: f64, t776: f64, t820: f64, t9607: f64, t16753: f64, t819: f64, t13087: f64, t13182: f64, t13190: f64, t13202: f64, t13208: f64, t13234: f64, t13237: f64, t13262: f64, t16836: f64, t2618: f64, t4172: f64, t4178: f64, t4184: f64, t4257: f64, t5587: f64, t5614: f64, t5619: f64, t817: f64, t843: f64, t9602: f64, t9672: f64, t9967: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t16839 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1456(t120, t5584);
        let (t16841, t16845, t16848, t16851, t16853, t16859) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1457(t16816, t16839, t4180, t4182, t5593, t9638, t5527, t776, t820, t9607, t16753, t819);
        let t16869 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1458(t13087, t13182, t13190, t13202, t13208, t13234, t13237, t13262, t16836, t16841, t16845, t16848, t16853, t16859, t2618, t4172, t4178, t4184, t4257, t5587, t5614, t5619, t817, t843, t9602, t9672, t9967);
    (t16839, t16841, t16845, t16851, t16853, t16859, t16869)
}
