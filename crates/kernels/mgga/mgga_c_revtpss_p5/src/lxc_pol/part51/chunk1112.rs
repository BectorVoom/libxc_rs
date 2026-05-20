//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1112/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1112<F: Float>(t121232: F, t125662: F, t125606: F, t32710: F, t1353: F, t1903: F, t120956: F, t1414: F, t828: F, t121000: F, t121003: F, t121028: F, t121043: F, t121046: F, t125617: F, t125621: F, t125623: F, t125630: F, t125632: F, t125637: F, t125642: F, t125646: F, t125650: F, t125652: F, t125654: F, t125659: F) -> (F, F) {
    let t125663 = t121232 * t125662;
    let t125666 = t32710 * t125606;
    let t125668 = t1903 * t1353;
    let t125671 = t120956 * t1414 * t828 * t125668;
    let t125673 = F::cast_from(0.13223814266738539448e-3_f64) * t121000 + F::cast_from(0.66119071333692697238e-4_f64) * t121003 + F::cast_from(0.131760844872908846e-2_f64) * t125617 + F::cast_from(0.51405703062096148813e-1_f64) * t125621 - F::cast_from(0.28912093960683998207e-1_f64) * t125623 + F::cast_from(0.56468933516960933998e-3_f64) * t125630 + F::cast_from(0.527043379491635384e-2_f64) * t125632 + F::cast_from(0.28234466758480466999e-3_f64) * t125637 + F::cast_from(0.28234466758480466999e-3_f64) * t125642 - F::cast_from(0.28234466758480466999e-3_f64) * t125646 - F::cast_from(0.33059535666846348619e-4_f64) * t125650 - F::cast_from(0.14456046980341999104e-1_f64) * t125652 + F::cast_from(0.25702851531048074406e-1_f64) * t125654 + F::cast_from(0.1859366460452550541e-4_f64) * t121028 - F::cast_from(0.112937867033921868e-2_f64) * t125659 + F::cast_from(0.75291911355947911999e-4_f64) * t125663 - F::cast_from(0.17354086964223805049e-2_f64) * t121043 - t121046 + F::cast_from(0.50779446784275991476e-1_f64) * t125666 - F::cast_from(0.14874931683620404328e-2_f64) * t125671;
    (t125668, t125673)
}
