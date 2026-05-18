//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1142/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1142<F: Float>(t120046: F, t33721: F, t8486: F, t233: F, t25373: F, t4533: F, t27312: F, t8650: F, t27349: F, t119843: F, t119850: F, t119860: F, t119870: F, t119876: F, t119878: F, t119889: F, t119941: F, t120057: F, t120058: F, t126158: F, t126164: F, t126166: F, t126182: F, t231: F, t27286: F, t27287: F, t27357: F, t31794: F, t31817: F, t32463: F, t4423: F, t8471: F, t8472: F, t8649: F) -> F {
    let t126185 = t8486 * t120046 * t33721;
    let t126188 = t25373 * t233 * t4533;
    let t126197 = t8650 * t27312;
    let t126202 = t8650 * t27349;
    let t126205 = -t119843 + F::new(0.25702851531048074406e-1) * t119850 + F::new(0.112937867033921868e-2) * t126158 + F::new(0.28912093960683998208e-1) * t119860 - F::new(0.3718732920905101082e-4) * t126164 + F::new(0.66119071333692697238e-4) * t126166 + t119870 + F::new(0.131760844872908846e-2) * t119876 + F::new(0.527043379491635384e-2) * t119878 + F::new(0.3427184259906141157e1) * t120057 * t120058 * t27312 + F::new(0.34271842599061411569e1) * t120057 * t120058 * t27349 - F::new(0.22847895066040941046e1) * t32463 * t27357 * t27286 + F::new(0.112937867033921868e-2) * t126182 - F::new(0.34708173928447610099e-2) * t126185 + F::new(0.17347256376410398924e1) * t8472 * t126188 - F::new(0.11423947533020470523e1) * t8649 * t31817 * t8471 * t4423 * t231 - F::new(0.51405703062096148812e-1) * t119889 - F::new(0.17347256376410398924e1) * t31794 * t126197 + F::new(0.17347256376410398924e1) * t119941 * t27287 - F::new(0.17347256376410398924e1) * t31794 * t126202;
    t126205
}
