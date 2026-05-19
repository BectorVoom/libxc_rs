//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 646/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk646<F: Float>(t1424: F, t1904: F, t213: F, t3894: F, t3898: F, t3910: F, t3922: F, t5601: F, t5604: F, t561: F, t5715: F, t5719: F, t5723: F, t6889: F, t6896: F, t6919: F) -> F {
    let t6922 = t3894 - t3898 - F::cast_from(0.10975748638225852664e-1_f64) * t5601 + F::cast_from(0.10975748638225852664e-1_f64) * t5719 + t3910 + F::cast_from(0.19514881078765566038e-1_f64) * t5604 - F::cast_from(0.19514881078765566038e-1_f64) * t5723 - t3922 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t6889 * t561 - F::cast_from(0.13170898365871023197e1_f64) * t5715 * t1904 + F::cast_from(0.13170898365871023197e1_f64) * t1424 * t6896 - F::cast_from(0.65854491829355115987e0_f64) * t1424 * t6919;
    t6922
}
