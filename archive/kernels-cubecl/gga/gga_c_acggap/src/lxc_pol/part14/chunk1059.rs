//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1059/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1059<F: Float>(t33831: F, t33840: F, t33842: F, t33844: F, t33852: F, t33853: F, t36823: F, t38701: F, t38704: F, t38706: F, t38709: F, t38711: F, t38713: F, t38717: F, t38721: F, t38723: F, t38727: F) -> F {
    let t38729 = -F::cast_from(0.12579236915841660827e-2_f64) * t33831 - F::cast_from(0.42874018118069736972e-3_f64) * t38701 + F::cast_from(0.10718504529517434243e-2_f64) * t38704 - t33840 + F::cast_from(0.85748036236139473944e-3_f64) * t38706 - t33842 + t33844 + t33852 + F::cast_from(0.41930789719472202757e-3_f64) * t33853 + F::cast_from(0.80031500487063509015e-2_f64) * t38709 - F::cast_from(0.94344276868812456204e-3_f64) * t38711 + t36823 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t38713 + F::cast_from(0.18868855373762491241e-2_f64) * t38717 + F::cast_from(0.94344276868812456205e-2_f64) * t38721 - F::cast_from(0.31448092289604152068e-3_f64) * t38723 - F::cast_from(0.20965394859736101379e-3_f64) * t38727;
    t38729
}
