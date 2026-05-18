//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 772/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk772<F: Float>(t225: F, t7997: F, t1579: F, t2061: F, t7071: F, t1558: F, t231: F, t7076: F, t233: F, t1957: F, t1580: F, t1956: F, t2067: F, t213: F, t257: F, t7070: F, t7387: F, t7390: F, t7403: F, t7409: F, t7411: F, t7766: F) -> (F, F, F, F, F, F, F, F) {
    let t7998 = t7997 * t225;
    let t8006 = t2061 * t1579;
    let t8007 = t7071 * t8006;
    let t8011 = t2061 * t1558 * t231;
    let t8012 = t7076 * t8011;
    let t8015 = t233 * t7997;
    let t8016 = t1957 * t8015;
    let t8019 = -t7387 + t7390 + F::new(0.65854491829355115987e0) * t213 * t7998 * t257 - F::new(0.65854491829355115987e0) * t7403 * t1580 + t7409 - t7411 - F::new(0.4336814094102599731e0) * t7766 * t2067 + F::new(0.8673628188205199462e0) * t7070 * t8007 + F::new(0.4336814094102599731e0) * t7070 * t8012 - F::new(0.4336814094102599731e0) * t1956 * t8016;
    (t7998, t8006, t8007, t8011, t8012, t8015, t8016, t8019)
}
