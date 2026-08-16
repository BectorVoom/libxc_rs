//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 997/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk997(t315: f64, t32123: f64, t1619: f64, t309: f64, t620: f64, t524: f64, t943: f64, t944: f64, t1222: f64, t1264: f64, t1620: f64, t2146: f64, t2147: f64, t2159: f64, t2331: f64, t32001: f64, t32006: f64, t32012: f64, t33715: f64, t33726: f64, t33727: f64, t33735: f64, t33739: f64, t7931: f64, t7932: f64, t8001: f64, t8400: f64, t8403: f64, t9010: f64, t9033: f64, t9058: f64) -> (f64, f64, f64) {
    let t33743 = t315 * t32123;
    let t33744 = t1619 * t309;
    let t33747 = 0.10408353825846239354e2_f64 * t33743 * t620 * t33744;
    let t33750 = t524 * t943;
    let t33751 = t33750 * t944;
    let t33755 = 0.13170898365871023197e1_f64 * t33715 + 0.13170898365871023197e1_f64 * t9010 * t1222 + 0.13170898365871023197e1_f64 * t32001 + 0.26341796731742046394e1_f64 * t8001 * t1620 + 0.34694512752820797848e1_f64 * t32006 - t33726 + 0.8673628188205199462e0_f64 * t33727 * t8403 + 0.8673628188205199462e0_f64 * t32012 + 0.8673628188205199462e0_f64 * t2146 * t2147 * t2331 * t1264 - 0.8673628188205199462e0_f64 * t8400 * t9033 * t33735 - 0.8673628188205199462e0_f64 * t7931 * t7932 * t33739 - t33747 - 0.8673628188205199462e0_f64 * t9058 * t2159 - 0.26020884564615598386e1_f64 * t8400 * t9033 * t33751;
    (t33750, t33751, t33755)
}
