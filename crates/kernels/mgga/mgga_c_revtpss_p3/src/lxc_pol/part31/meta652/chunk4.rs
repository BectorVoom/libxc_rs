//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2168/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2168<F: Float>(t29807: F, t994: F, t1000: F, t100596: F, t107226: F, t107268: F, t1096: F, t1652: F, t19491: F, t19548: F, t25464: F, t25699: F, t27419: F, t27568: F, t27661: F, t27680: F, t27688: F, t29727: F, t29751: F, t29809: F, t29848: F, t29883: F, t4764: F, t5015: F, t7145: F, t7151: F, t7159: F, t7160: F, t7828: F, t93490: F, t93890: F, t93893: F, t94080: F, t94081: F, t989: F, t99684: F, t999: F, t99947: F) -> F {
    let t107566 = t994 * t29807;
    let t107603 = F::cast_from(0.13170898365871023197e1_f64) * t27568 * t4764 - F::cast_from(0.52041769129231196772e1_f64) * t25699 * t7145 * t29727 * t999 - F::cast_from(0.65854491829355115987e0_f64) * t107566 * t1000 + F::cast_from(0.17347256376410398924e1_f64) * t27419 * t27688 + F::cast_from(0.65854491829355115987e0_f64) * t989 * t29809 - F::cast_from(0.17347256376410398924e1_f64) * t94080 * t107226 * t94081 - F::cast_from(0.13170898365871023197e1_f64) * t99947 * t1652 - F::cast_from(0.17347256376410398924e1_f64) * t27661 * t27680 + F::cast_from(0.52041769129231196772e1_f64) * t7151 * t25464 * t29751 * t999 - F::cast_from(0.8673628188205199462e0_f64) * t93490 * t29848 - F::cast_from(0.26020884564615598386e1_f64) * t100596 * t107268 * t19548 + F::cast_from(0.26020884564615598386e1_f64) * t99684 * t107268 * t19491 + F::cast_from(0.8673628188205199462e0_f64) * t93890 * t107226 * t93893 - F::cast_from(0.17347256376410398924e1_f64) * t7151 * t7160 * t29883 * t1096 - F::cast_from(0.52041769129231196772e1_f64) * t7159 * t25464 * t7828 * t5015;
    t107603
}
