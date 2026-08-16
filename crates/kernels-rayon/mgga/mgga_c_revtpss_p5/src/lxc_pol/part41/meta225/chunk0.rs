//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 873/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk873(t225: f64, t6041: f64, t1579: f64, t2770: f64, t1559: f64, t213: f64, t234: f64, t2776: f64, t2780: f64, t2796: f64, t2810: f64, t2811: f64, t4497: f64, t4501: f64, t4520: f64, t4524: f64, t4526: f64, t5978: f64, t6017: f64, t6022: f64, t820: f64, t879: f64) -> (f64, f64, f64, f64) {
    let t6042 = t6041 * t225;
    let t6048 = t1579 * t1579;
    let t6049 = t2770 * t6048;
    let t6071 = t2776 - t2780 + 0.10975748638225852664e-1_f64 * t4497 - 0.10975748638225852664e-1_f64 * t4520 + t2796 - 0.19514881078765566038e-1_f64 * t4501 + 0.19514881078765566038e-1_f64 * t4524 - t2810 + 0.13170898365871023197e1_f64 * t820 * t2811 * t6022 - 0.13170898365871023197e1_f64 * t820 * t4526 * t1559 - 0.65854491829355115987e0_f64 * t820 * t879 * t6017 - 0.65854491829355115987e0_f64 * t820 * t879 * t5978 + 0.65854491829355115987e0_f64 * t213 * t234 * t6041;
    (t6042, t6048, t6049, t6071)
}
