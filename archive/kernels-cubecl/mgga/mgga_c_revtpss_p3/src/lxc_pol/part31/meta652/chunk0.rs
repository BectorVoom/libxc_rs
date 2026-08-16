//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2164/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2164<F: Float>(t4746: F, t7143: F, t1646: F, t1695: F, t100691: F, t100760: F, t1043: F, t107201: F, t1089: F, t1096: F, t1652: F, t1668: F, t19385: F, t19425: F, t1985: F, t225: F, t25629: F, t27422: F, t27423: F, t27568: F, t27587: F, t27599: F, t27679: F, t29747: F, t29759: F, t29887: F, t342: F, t385: F, t4941: F, t7102: F, t7140: F, t7151: F, t7160: F, t7837: F, t93436: F, t93438: F, t93921: F, t999: F, t99934: F) -> F {
    let t107358 = t4746 * t7143;
    let t107392 = t1646 * t1695;
    let t107405 = -F::cast_from(0.17347256376410398924e1_f64) * t107358 * t27423 - F::cast_from(0.17347256376410398924e1_f64) * t25629 * t27679 * t1668 * t1089 - F::cast_from(0.17347256376410398924e1_f64) * t25629 * t27422 * t1668 * t1089 + F::cast_from(0.34694512752820797848e1_f64) * t93436 * t29759 * t93438 - F::cast_from(0.8673628188205199462e0_f64) * t27587 * t7837 - F::cast_from(0.34694512752820797848e1_f64) * t7151 * t7160 * t29887 * t999 - F::cast_from(0.17347256376410398924e1_f64) * t25629 * t29747 * t1043 * t1089 + F::cast_from(0.65854491829355115987e0_f64) * t7102 * t19385 - F::cast_from(0.13170898365871023197e1_f64) * t100760 * t1652 - F::cast_from(0.39512695097613069591e1_f64) * t7140 * t19425 + F::cast_from(0.34694512752820797848e1_f64) * t99934 * t27599 + F::cast_from(0.13170898365871023197e1_f64) * t27568 * t4941 - F::cast_from(0.69389025505641595696e1_f64) * t93921 * t1985 * t107392 * t999 - F::cast_from(0.10408353825846239354e2_f64) * t100691 * t1985 * t107392 * t1096 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t107201 * t225 * t385;
    t107405
}
