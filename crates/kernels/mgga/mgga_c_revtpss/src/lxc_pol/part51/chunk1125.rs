//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1125/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1125<F: Float>(t120980: F, t1873: F, t32265: F, t32269: F, t125849: F, t552: F, t8590: F, t1405: F, t33959: F, t121336: F, t121339: F, t121343: F, t121347: F, t121350: F, t121356: F, t121364: F, t121366: F, t125901: F, t125903: F, t125906: F, t125915: F, t125918: F, t1955: F, t32233: F, t32262: F, t32700: F, t33947: F, t34231: F, t7274: F, t7930: F, t8579: F) -> F {
    let t125922 = t120980 * t1873;
    let t125923 = t32265 * t125922;
    let t125925 = t32269 * t125922;
    let t125928 = t125849 * t8590 * t552;
    let t125930 = t33959 * t1405;
    let t125932 = F::cast_from(0.527043379491635384e-2_f64) * t121336 - F::cast_from(0.17135921299530705785e1_f64) * t32700 * t33947 + F::cast_from(0.51405703062096148812e-1_f64) * t121339 + F::cast_from(0.18822977838986977999e-4_f64) * t125901 - F::cast_from(0.33467254597718846885e-4_f64) * t125903 - t121343 + t121347 + t121350 + F::cast_from(0.17347256376410398924e1_f64) * t8579 * t125906 - F::cast_from(0.17347256376410398924e1_f64) * t1955 * t7274 * t7930 - F::cast_from(0.34708173928447610099e-2_f64) * t121356 + F::cast_from(0.11423947533020470523e1_f64) * t34231 * t32262 - F::cast_from(0.17347256376410398924e1_f64) * t32233 * t125915 + F::cast_from(0.8673628188205199462e0_f64) * t32233 * t125918 + t121364 - F::cast_from(0.76169170176413987214e-1_f64) * t121366 - F::cast_from(0.74374658418102021639e-4_f64) * t125923 + F::cast_from(0.13223814266738539448e-3_f64) * t125925 - F::cast_from(0.1859366460452550541e-3_f64) * t125928 + F::cast_from(0.86770434821119025247e-3_f64) * t125930;
    t125932
}
