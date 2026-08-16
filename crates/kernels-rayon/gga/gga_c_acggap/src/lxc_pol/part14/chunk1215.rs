//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1215/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1215(t8998: f64, t9076: f64, t1938: f64, t7973: f64, t2131: f64, t2132: f64, t309: f64, t9767: f64, t1659: f64, t2127: f64, t2351: f64, t32061: f64, t32073: f64, t33771: f64, t33783: f64, t33786: f64, t33789: f64, t38827: f64, t6558: f64, t7912: f64, t7932: f64, t8400: f64, t9010: f64, t9058: f64, t9790: f64) -> f64 {
    let t40793 = t8998 * t9076;
    let t40796 = t7973 * t1938;
    let t40803 = t2131 * t2132 * t9767 * t309;
    let t40815 = -0.34694512752820797848e1_f64 * t40793 + 0.26341796731742046394e1_f64 * t33771 - 0.65854491829355115987e0_f64 * t40796 - 0.8673628188205199462e0_f64 * t9058 * t2351 + 0.52041769129231196772e1_f64 * t32061 - 0.8673628188205199462e0_f64 * t40803 - 0.13170898365871023197e1_f64 * t9010 * t1659 + t33783 - t32073 + 0.17347256376410398924e1_f64 * t7912 * t9790 - 0.65854491829355115987e0_f64 * t2127 * t6558 - t33786 + 0.17347256376410398924e1_f64 * t33789 + 0.8673628188205199462e0_f64 * t8400 * t7932 * t38827;
    t40815
}
