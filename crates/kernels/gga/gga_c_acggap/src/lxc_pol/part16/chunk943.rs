//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 943/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk943<F: Float>(t1165: F, t5572: F, t7575: F, t8600: F, t30120: F, t9645: F, t1815: F, t1992: F, t30127: F, t7842: F, t33831: F, t33840: F, t33842: F, t33844: F, t33852: F, t33853: F, t36823: F, t38701: F, t38704: F, t38706: F, t38709: F, t38711: F, t38713: F, t38717: F) -> (F,) {
    let t38721 = t7575 * t1165 * t8600 * t5572;
    let t38723 = t30120 * t9645;
    let t38727 = t30127 * t7842 * t1992 * t1815;
    let t38729 = -0.12579236915841660827e-2 * t33831 - 0.42874018118069736972e-3 * t38701 + 0.10718504529517434243e-2 * t38704 - t33840 + 0.85748036236139473944e-3 * t38706 - t33842 + t33844 + t33852 + 0.41930789719472202757e-3 * t33853 + 0.80031500487063509015e-2 * t38709 - 0.94344276868812456204e-3 * t38711 + t36823 + 7.0 / 72.0 * t38713 + 0.18868855373762491241e-2 * t38717 + 0.94344276868812456205e-2 * t38721 - 0.31448092289604152068e-3 * t38723 - 0.20965394859736101379e-3 * t38727;
    (t38729,)
}
