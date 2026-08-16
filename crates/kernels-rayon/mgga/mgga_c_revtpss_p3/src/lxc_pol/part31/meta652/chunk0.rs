//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2164/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2164(t4746: f64, t7143: f64, t1646: f64, t1695: f64, t100691: f64, t100760: f64, t1043: f64, t107201: f64, t1089: f64, t1096: f64, t1652: f64, t1668: f64, t19385: f64, t19425: f64, t1985: f64, t225: f64, t25629: f64, t27422: f64, t27423: f64, t27568: f64, t27587: f64, t27599: f64, t27679: f64, t29747: f64, t29759: f64, t29887: f64, t342: f64, t385: f64, t4941: f64, t7102: f64, t7140: f64, t7151: f64, t7160: f64, t7837: f64, t93436: f64, t93438: f64, t93921: f64, t999: f64, t99934: f64) -> f64 {
    let t107358 = t4746 * t7143;
    let t107392 = t1646 * t1695;
    let t107405 = -0.17347256376410398924e1_f64 * t107358 * t27423 - 0.17347256376410398924e1_f64 * t25629 * t27679 * t1668 * t1089 - 0.17347256376410398924e1_f64 * t25629 * t27422 * t1668 * t1089 + 0.34694512752820797848e1_f64 * t93436 * t29759 * t93438 - 0.8673628188205199462e0_f64 * t27587 * t7837 - 0.34694512752820797848e1_f64 * t7151 * t7160 * t29887 * t999 - 0.17347256376410398924e1_f64 * t25629 * t29747 * t1043 * t1089 + 0.65854491829355115987e0_f64 * t7102 * t19385 - 0.13170898365871023197e1_f64 * t100760 * t1652 - 0.39512695097613069591e1_f64 * t7140 * t19425 + 0.34694512752820797848e1_f64 * t99934 * t27599 + 0.13170898365871023197e1_f64 * t27568 * t4941 - 0.69389025505641595696e1_f64 * t93921 * t1985 * t107392 * t999 - 0.10408353825846239354e2_f64 * t100691 * t1985 * t107392 * t1096 + 0.65854491829355115987e0_f64 * t342 * t107201 * t225 * t385;
    t107405
}
