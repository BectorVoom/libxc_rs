//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 988/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk988(t7884: f64, t790: f64, t1134: f64, t1144: f64, t2112: f64, t2120: f64, t2146: f64, t2957: f64, t2965: f64, t2990: f64, t307: f64, t311: f64, t7805: f64, t7821: f64, t7825: f64, t7828: f64, t786: f64, t800: f64) -> (f64, f64) {
    let t7885 = t790 * t7884;
    let t7888 = 0.65854491829355115987e0_f64 * t7805 * t311 - 0.13170898365871023197e1_f64 * t2957 * t800 + 0.13170898365871023197e1_f64 * t1134 * t2120 - 0.65854491829355115987e0_f64 * t1134 * t2146 - 0.65854491829355115987e0_f64 * t2112 * t1144 + 0.26341796731742046394e1_f64 * t786 * t2965 - 0.13170898365871023197e1_f64 * t786 * t2990 - 0.39512695097613069591e1_f64 * t307 * t7821 + 0.26341796731742046394e1_f64 * t307 * t7825 + 0.13170898365871023197e1_f64 * t307 * t7828 - 0.65854491829355115987e0_f64 * t307 * t7885;
    (t7885, t7888)
}
