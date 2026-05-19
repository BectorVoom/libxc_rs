//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1139/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1139<F: Float>(t121099: F, t1381: F, t8590: F, t27: F, t3999: F, t8589: F, t25875: F, t4021: F, t32268: F, t240: F, t31752: F, t545: F, t843: F) -> (F, F, F, F, F, F) {
    let t121101 = t121099 * t8590 * t1381;
    let t121102 = F::cast_from(0.66119071333692697238e-4_f64) * t121101;
    let t121106 = t8589 * t3999 * t27;
    let t121107 = t25875 * t121106;
    let t121108 = t121107 * t4021;
    let t121109 = F::cast_from(0.7437465841810202164e-4_f64) * t121108;
    let t121110 = t32268 * t121106;
    let t121111 = t121110 * t4021;
    let t121112 = F::cast_from(0.13223814266738539448e-3_f64) * t121111;
    let t121116 = t31752 * t545 * t843 * t240;
    (t121102, t121107, t121109, t121110, t121112, t121116)
}
