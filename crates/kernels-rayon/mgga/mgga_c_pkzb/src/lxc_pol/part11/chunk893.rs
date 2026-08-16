//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 893/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk893(t790: f64, t9712: f64, t1134: f64, t1144: f64, t2957: f64, t2965: f64, t2990: f64, t307: f64, t311: f64, t3670: f64, t3676: f64, t3695: f64, t786: f64, t800: f64, t9634: f64, t9648: f64, t9651: f64, t9657: f64) -> (f64, f64) {
    let t9713 = t790 * t9712;
    let t9716 = 0.65854491829355115987e0_f64 * t9634 * t311 - 0.65854491829355115987e0_f64 * t3670 * t800 - 0.13170898365871023197e1_f64 * t2957 * t1144 + 0.26341796731742046394e1_f64 * t1134 * t2965 - 0.13170898365871023197e1_f64 * t1134 * t2990 + 0.13170898365871023197e1_f64 * t786 * t3676 - 0.39512695097613069591e1_f64 * t307 * t9648 + 0.26341796731742046394e1_f64 * t307 * t9651 - 0.65854491829355115987e0_f64 * t786 * t3695 + 0.13170898365871023197e1_f64 * t307 * t9657 - 0.65854491829355115987e0_f64 * t307 * t9713;
    (t9713, t9716)
}
