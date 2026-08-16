//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1281;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta438(t15338: f64, t18409: f64, t3447: f64, t20217: f64, t3450: f64, t18469: f64, t52059: f64, t4904: f64, t64763: f64, t18532: f64, t4889: f64, t1174: f64, t135: f64, t22040: f64, t18321: f64, t4916: f64, t11583: f64, t21510: f64, t11570: f64, t15419: f64, t21745: f64, t20234: f64, t44505: f64, t1171: f64, t22104: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73395, t73405, t73417, t73420, t73424, t73427) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1281(t15338, t18409, t3447, t20217, t3450, t18469, t52059, t4904, t64763, t18532, t4889, t1174, t135, t22040);
        let (t73433, t73444, t73451, t73491, t73496, t73523) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1282(t18321, t4916, t11583, t21510, t11570, t15419, t21745, t3447, t20234, t44505, t1171, t22104);
    (t73395, t73405, t73417, t73420, t73424, t73427, t73433, t73444, t73451, t73491, t73496, t73523)
}
