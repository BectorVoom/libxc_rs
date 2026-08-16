//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2216/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2216<F: Float>(t15734: F, t25522: F, t15816: F, t7121: F, t15822: F, t25504: F, t15794: F, t25580: F, t1047: F, t15959: F, t16104: F, t25517: F, t27450: F, t3136: F, t3157: F, t4783: F, t4825: F, t93646: F, t93673: F, t93683: F, t93685: F, t93752: F, t93821: F) -> F {
    let t100166 = t25522 * t15734;
    let t100168 = t15816 * t7121;
    let t100173 = t15822 * t25504;
    let t100186 = F::cast_from(0.57165357490759649296e-3_f64) * t25580 * t15794;
    let t100187 = F::cast_from(0.30488190661738479624e-2_f64) * t93646 * t4825 - F::cast_from(0.3811023832717309953e-3_f64) * t100166 + F::cast_from(0.85748036236139473944e-3_f64) * t100168 * t1047 + F::cast_from(0.42874018118069736972e-3_f64) * t27450 * t3136 + F::cast_from(0.85748036236139473944e-3_f64) * t100173 * t3157 + F::cast_from(0.57165357490759649296e-3_f64) * t93821 * t4783 + F::cast_from(0.57165357490759649296e-3_f64) * t25517 * t15959 - F::cast_from(0.20325460441158986416e-2_f64) * t93673 - F::cast_from(0.57165357490759649296e-3_f64) * t93752 * t16104 - F::cast_from(0.57165357490759649296e-3_f64) * t93683 - F::cast_from(0.28582678745379824648e-3_f64) * t93685 - t100186;
    t100187
}
