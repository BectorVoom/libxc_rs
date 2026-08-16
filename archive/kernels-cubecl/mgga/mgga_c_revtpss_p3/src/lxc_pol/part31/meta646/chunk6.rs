//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2118/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2118<F: Float>(t106005: F, t106020: F, t106028: F, t106055: F, t106067: F, t106078: F, t106092: F, t106108: F, t105974: F, t105976: F, t1580: F, t213: F, t225: F, t25322: F, t257: F, t6049: F, t92895: F, t92905: F, t98875: F, t98879: F, t98881: F, t98894: F, t98897: F, t98907: F, t98911: F, t99429: F) -> (F, F) {
    let t106111 = t106005 + t106020 + t106028 + t106055 + t106067 + t106078 + t106092 + t106108;
    let t106116 = F::cast_from(0.17135234354032049604e-2_f64) * t92895 + F::cast_from(0.43368140941025997311e-1_f64) * t105974 - F::cast_from(0.77108554593144223219e-1_f64) * t105976 + F::cast_from(0.48186823267806663678e-3_f64) * t92905 + F::cast_from(0.45699670022203476294e-2_f64) * t98875 - t98879 + t98881 + F::cast_from(0.13170898365871023197e1_f64) * t25322 * t6049 - F::cast_from(0.13170898365871023197e1_f64) * t99429 * t1580 + t98894 - t98897 + t98907 - t98911 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t106111 * t225 * t257;
    (t106111, t106116)
}
