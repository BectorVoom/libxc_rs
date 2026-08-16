//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3793/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3793(t17306: f64, t488: f64, t1269: f64, t6564: f64, t1210: f64, t12641: f64, t1277: f64, t1295: f64, t17973: f64, t17974: f64, t17975: f64, t17992: f64, t17995: f64, t18042: f64, t18062: f64, t18103: f64, t20722: f64, t20741: f64, t21394: f64, t21408: f64, t3561: f64, t3572: f64, t3585: f64, t5225: f64, t5231: f64, t5245: f64, t5423: f64, t5497: f64, t59464: f64) -> f64 {
    let t72927 = t17306 * t488;
    let t72933 = t6564 * t1269;
    let t72956 = -0.52683593463484092788e1_f64 * t72927 * t17975 - 0.52683593463484092788e1_f64 * t17973 * t17974 * t18042 - 0.13170898365871023197e1_f64 * t72933 * t1295 + 0.52683593463484092788e1_f64 * t12641 * t20722 + 0.26341796731742046394e1_f64 * t1210 * t1277 * t5245 * t5497 - 0.26341796731742046394e1_f64 * t3572 * t20741 - 0.26341796731742046394e1_f64 * t17995 * t18103 + 0.52683593463484092788e1_f64 * t59464 * t5231 + 0.26341796731742046394e1_f64 * t18062 * t5423 - 0.13170898365871023197e1_f64 * t21394 * t3585 + 0.26341796731742046394e1_f64 * t5225 * t17992 + 0.52683593463484092788e1_f64 * t3561 * t21408;
    t72956
}
