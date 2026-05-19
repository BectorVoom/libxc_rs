//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1060/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1060<F: Float>(t30624: F, t34522: F, t34526: F, t34529: F, t34532: F, t34535: F, t34538: F, t34539: F, t34541: F, t34543: F, t34545: F, t34547: F, t34549: F, t34553: F, t34557: F, t34559: F, t34562: F, t34563: F) -> F {
    let t34565 = F::cast_from(0.18868855373762491241e-2_f64) * t34522 + F::cast_from(0.41930789719472202758e-3_f64) * t34526 + t34529 / F::new(48.0) + t34532 / F::new(48.0) - t34535 + F::cast_from(0.42874018118069736972e-3_f64) * t30624 + t34538 - F::cast_from(0.17149607247227894789e-2_f64) * t34539 + F::cast_from(0.25724410870841842183e-2_f64) * t34541 - F::cast_from(0.17149607247227894789e-1_f64) * t34543 + F::cast_from(0.51448821741683684367e-2_f64) * t34545 - F::cast_from(0.17149607247227894789e-2_f64) * t34547 - F::cast_from(0.80031500487063509014e-2_f64) * t34549 + F::cast_from(0.94344276868812456204e-3_f64) * t34553 + t34557 + F::cast_from(0.31448092289604152068e-2_f64) * t34559 + t34562 + F::cast_from(0.13719685797782315831e-1_f64) * t34563;
    t34565
}
