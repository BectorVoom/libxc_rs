//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2160/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2160(t107201: f64, t1096: f64, t1695: f64, t1983: f64, t1984: f64, t20152: f64, t20188: f64, t25591: f64, t25692: f64, t27419: f64, t27550: f64, t27576: f64, t27616: f64, t27679: f64, t27699: f64, t27703: f64, t29747: f64, t29875: f64, t29887: f64, t359: f64, t4773: f64, t4947: f64, t5015: f64, t5016: f64, t6259: f64, t7140: f64, t7144: f64, t7145: f64, t7151: f64, t7160: f64, t7821: f64, t94026: f64, t988: f64, t999: f64, t99934: f64) -> f64 {
    let t107206 = 0.34694512752820797848e1_f64 * t7144 * t7160 * t29887 * t988 - 0.34694512752820797848e1_f64 * t7151 * t7160 * t7821 * t5015 - 0.65854491829355115987e0_f64 * t7140 * t20152 + 0.34694512752820797848e1_f64 * t99934 * t27703 + 0.34694512752820797848e1_f64 * t7144 * t7160 * t29747 * t1096 - 0.13170898365871023197e1_f64 * t27550 * t4773 + 0.17347256376410398924e1_f64 * t25591 * t7145 * t29875 * t999 - 0.13170898365871023197e1_f64 * t27616 * t5016 + 0.26341796731742046394e1_f64 * t27616 * t4947 - 0.39512695097613069591e1_f64 * t94026 * t20188 + 0.34694512752820797848e1_f64 * t7144 * t7160 * t27679 * t1695 - 0.65854491829355115987e0_f64 * t25692 * t6259 + 0.26341796731742046394e1_f64 * t27699 * t4947 - 0.34694512752820797848e1_f64 * t27419 * t27576 - 0.4336814094102599731e0_f64 * t1983 * t1984 * t359 * t107201;
    t107206
}
