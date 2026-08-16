//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3795/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3795(t1210: f64, t1214: f64, t12673: f64, t1274: f64, t1277: f64, t1294: f64, t17973: f64, t17974: f64, t17986: f64, t17987: f64, t17998: f64, t18018: f64, t18062: f64, t18097: f64, t20697: f64, t20744: f64, t21617: f64, t21621: f64, t3576: f64, t3585: f64, t3737: f64, t45433: f64, t5246: f64, t56707: f64, t6574: f64, t6703: f64, t6745: f64) -> f64 {
    let t73020 = -0.65854491829355115987e0_f64 * t21621 * t3585 + 0.26341796731742046394e1_f64 * t1274 * t3737 * t21617 * t1294 - 0.52683593463484092788e1_f64 * t56707 * t20744 - 0.26341796731742046394e1_f64 * t18062 * t5246 - 0.52683593463484092788e1_f64 * t17986 * t17987 * t18018 - 0.26341796731742046394e1_f64 * t18097 * t5246 - 0.26341796731742046394e1_f64 * t17973 * t17974 * t17998 - 0.65854491829355115987e0_f64 * t12673 * t6745 + 0.13170898365871023197e1_f64 * t20697 * t3576 + 0.13170898365871023197e1_f64 * t1210 * t1277 * t21617 * t1214 + 0.13170898365871023197e1_f64 * t12673 * t6703 + 0.26341796731742046394e1_f64 * t45433 * t6574;
    t73020
}
