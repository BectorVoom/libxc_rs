//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 665/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk665<F: Float>(t1424: F, t1445: F, t213: F, t3894: F, t3898: F, t3901: F, t3904: F, t3910: F, t3912: F, t3918: F, t3922: F, t4067: F, t4071: F, t4078: F, t4132: F, t561: F) -> (F,) {
    let t4135 = t3894 - t3898 - 0.10975748638225852664e-1 * t3901 + 0.10975748638225852664e-1 * t3904 + t3910 + 0.19514881078765566038e-1 * t3912 - 0.19514881078765566038e-1 * t3918 - t3922 + 0.65854491829355115987e0 * t213 * t4067 * t561 - 0.13170898365871023197e1 * t4071 * t1445 + 0.13170898365871023197e1 * t1424 * t4078 - 0.65854491829355115987e0 * t1424 * t4132;
    (t4135,)
}
