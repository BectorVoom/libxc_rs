//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1746/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1746<F: Float>(t1469: F, t1774: F, t17643: F, t5819: F, t24494: F, t5192: F, t68255: F, t81156: F, t81158: F, t89824: F, t89828: F, t89832: F, t89839: F, t89843: F, t89847: F, t89851: F, t89855: F) -> (F, F, F, F) {
    let t90253 = t1469 * t1774;
    let t90262 = t17643 * t5819;
    let t90293 = F::cast_from(0.4155806185363551302e3_f64) * t5192 * t24494;
    let t90305 = F::cast_from(0.61805555555555555555e-1_f64) * t89824 - F::cast_from(0.22249999999999999999e0_f64) * t89828 - F::cast_from(0.27469135802469135803e-1_f64) * t89832 + F::cast_from(0.24722222222222222222e-1_f64) * t81156 - F::cast_from(0.74166666666666666668e-1_f64) * t81158 + F::cast_from(0.24722222222222222222e-1_f64) * t68255 - F::cast_from(0.18541666666666666666e-1_f64) * t89839 - F::cast_from(0.24722222222222222222e-1_f64) * t89843 + F::new(0.33375e0) * t89847 + F::cast_from(0.55625000000000000001e-1_f64) * t89851 + F::cast_from(0.74166666666666666668e-1_f64) * t89855;
    (t90253, t90262, t90293, t90305)
}
