//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2184/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2184<F: Float>(t22279: F, t28167: F, t8996: F, t29506: F, t7313: F, t105850: F, t105859: F, t105863: F, t105866: F, t105889: F, t105894: F, t105897: F, t107881: F, t108062: F, t118: F, t1310: F, t13426: F, t18220: F, t18227: F, t18232: F, t18245: F, t1932: F, t2007: F, t21658: F, t29573: F, t508: F, t5884: F, t671: F, t6765: F, t6983: F, t6985: F, t7007: F, t7221: F, t7746: F) -> F {
    let t108067 = F::new(12.0) * t28167 * t8996 * t22279;
    let t108068 = t29506 * t7313;
    let t108071 = -F::new(2.0) * t18220 * t2007 - F::new(2.0) * t5884 * t7221 - F::new(2.0) * t105850 * t508 - F::new(2.0) * t29573 * t1310 - t6983 * t6765 - t1932 * t21658 - t105859 - F::new(2.0) * t6985 * t18232 - t105863 - F::new(2.0) * t18245 * t7007 - F::new(2.0) * t105866 * t671 - t105889 - F::new(4.0) * t13426 * t7746 + t105894 + t105897 - t118 * (t107881 + t108062) + t108067 + t108068 - F::new(4.0) * t18227 * t7746;
    t108071
}
