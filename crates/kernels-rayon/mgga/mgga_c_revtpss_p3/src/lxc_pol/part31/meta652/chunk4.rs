//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2168/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2168(t29807: f64, t994: f64, t1000: f64, t100596: f64, t107226: f64, t107268: f64, t1096: f64, t1652: f64, t19491: f64, t19548: f64, t25464: f64, t25699: f64, t27419: f64, t27568: f64, t27661: f64, t27680: f64, t27688: f64, t29727: f64, t29751: f64, t29809: f64, t29848: f64, t29883: f64, t4764: f64, t5015: f64, t7145: f64, t7151: f64, t7159: f64, t7160: f64, t7828: f64, t93490: f64, t93890: f64, t93893: f64, t94080: f64, t94081: f64, t989: f64, t99684: f64, t999: f64, t99947: f64) -> f64 {
    let t107566 = t994 * t29807;
    let t107603 = 0.13170898365871023197e1_f64 * t27568 * t4764 - 0.52041769129231196772e1_f64 * t25699 * t7145 * t29727 * t999 - 0.65854491829355115987e0_f64 * t107566 * t1000 + 0.17347256376410398924e1_f64 * t27419 * t27688 + 0.65854491829355115987e0_f64 * t989 * t29809 - 0.17347256376410398924e1_f64 * t94080 * t107226 * t94081 - 0.13170898365871023197e1_f64 * t99947 * t1652 - 0.17347256376410398924e1_f64 * t27661 * t27680 + 0.52041769129231196772e1_f64 * t7151 * t25464 * t29751 * t999 - 0.8673628188205199462e0_f64 * t93490 * t29848 - 0.26020884564615598386e1_f64 * t100596 * t107268 * t19548 + 0.26020884564615598386e1_f64 * t99684 * t107268 * t19491 + 0.8673628188205199462e0_f64 * t93890 * t107226 * t93893 - 0.17347256376410398924e1_f64 * t7151 * t7160 * t29883 * t1096 - 0.52041769129231196772e1_f64 * t7159 * t25464 * t7828 * t5015;
    t107603
}
