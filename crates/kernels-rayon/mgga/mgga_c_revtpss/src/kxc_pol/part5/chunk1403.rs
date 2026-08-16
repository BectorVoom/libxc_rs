//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1403/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1403(t1385: f64, t6888: f64, t10070: f64, t10074: f64, t1399: f64, t14191: f64, t14193: f64, t14203: f64, t14209: f64, t14255: f64, t1883: f64, t213: f64, t21981: f64, t22005: f64, t22009: f64, t22016: f64, t22307: f64, t22316: f64, t4118: f64, t546: f64, t5659: f64, t5675: f64, t5745: f64, t5755: f64, t5767: f64, t6874: f64, t820: f64) -> f64 {
    let t22321 = t1385 * t6888;
    let t22325 = -0.13170898365871023197e1_f64 * t820 * t14255 * t1883 - 0.13170898365871023197e1_f64 * t820 * t5767 * t5659 - 0.65854491829355115987e0_f64 * t5755 * t22005 * t1399 + 0.13170898365871023197e1_f64 * t5745 * t22009 * t5675 + 0.26341796731742046394e1_f64 * t5745 * t21981 * t5675 - 0.39512695097613069591e1_f64 * t14193 * t22005 * t22016 + 0.65854491829355115987e0_f64 * t213 * t546 * t22307 - 0.65854491829355115987e0_f64 * t820 * t4118 * t6874 + 0.19514881078765566037e-1_f64 * t22316 - t14191 - 0.13009920719177044025e-2_f64 * t14203 + t14209 - 0.73171657588172351096e-2_f64 * t10070 + 0.65049603595885220126e-3_f64 * t10074 - 0.65854491829355115987e0_f64 * t820 * t22321 * t1399;
    t22325
}
