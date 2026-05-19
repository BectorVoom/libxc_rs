//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1225/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1225<F: Float>(t115673: F, t115687: F, t115698: F, t115712: F, t103521: F, t103547: F, t110256: F, t110639: F, t110677: F, t110679: F, t1580: F, t1956: F, t1957: F, t213: F, t225: F, t231: F, t233: F, t23414: F, t257: F, t27199: F, t30396: F, t30411: F, t6016: F, t6071: F, t7070: F, t7071: F, t7076: F, t7403: F, t7997: F, t95914: F, t95930: F) -> F {
    let t115714 = t115673 + t115687 + t115698 + t115712;
    let t115744 = F::cast_from(0.58544643236296698113e-1_f64) * t110639 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t115714 * t225 * t257 + t95914 - t95930 - F::cast_from(0.4336814094102599731e0_f64) * t1956 * t1957 * t233 * t115714 - F::cast_from(0.19756347548806534796e1_f64) * t110256 * t1580 + F::cast_from(0.57824187921367996415e-1_f64) * t103521 - F::cast_from(0.39512695097613069591e1_f64) * t7403 * t23414 + F::cast_from(0.43368140941025997312e-1_f64) * t110677 - F::cast_from(0.77108554593144223218e-1_f64) * t110679 + F::cast_from(0.13010442282307799193e1_f64) * t7070 * t7076 * t7997 * t6016 * t231 + F::cast_from(0.13010442282307799193e1_f64) * t27199 * t30396 + F::cast_from(0.26020884564615598386e1_f64) * t7070 * t7071 * t7997 * t6071 - F::cast_from(0.78062653693846795158e1_f64) * t27199 * t30411 - F::cast_from(0.28912093960683998208e-1_f64) * t103547;
    t115744
}
