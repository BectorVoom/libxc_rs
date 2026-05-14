//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1019/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1019<F: Float>(t125849: F, t552: F, t8590: F, t1405: F, t33959: F, t121336: F, t121339: F, t121343: F, t121347: F, t121350: F, t121356: F, t121364: F, t121366: F, t125901: F, t125903: F, t125906: F, t125915: F, t125918: F, t125923: F, t125925: F, t1955: F, t32233: F, t32262: F, t32700: F, t33947: F, t34231: F, t7274: F, t7930: F, t8579: F) -> (F,) {
    let t125928 = t125849 * t8590 * t552;
    let t125930 = t33959 * t1405;
    let t125932 = 0.527043379491635384e-2 * t121336 - 0.17135921299530705785e1 * t32700 * t33947 + 0.51405703062096148812e-1 * t121339 + 0.18822977838986977999e-4 * t125901 - 0.33467254597718846885e-4 * t125903 - t121343 + t121347 + t121350 + 0.17347256376410398924e1 * t8579 * t125906 - 0.17347256376410398924e1 * t1955 * t7274 * t7930 - 0.34708173928447610099e-2 * t121356 + 0.11423947533020470523e1 * t34231 * t32262 - 0.17347256376410398924e1 * t32233 * t125915 + 0.8673628188205199462e0 * t32233 * t125918 + t121364 - 0.76169170176413987214e-1 * t121366 - 0.74374658418102021639e-4 * t125923 + 0.13223814266738539448e-3 * t125925 - 0.1859366460452550541e-3 * t125928 + 0.86770434821119025247e-3 * t125930;
    (t125932,)
}
