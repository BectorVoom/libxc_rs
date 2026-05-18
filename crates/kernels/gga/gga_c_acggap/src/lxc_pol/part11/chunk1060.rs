//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1060/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1060<F: Float>(t30624: F, t34522: F, t34526: F, t34529: F, t34532: F, t34535: F, t34538: F, t34539: F, t34541: F, t34543: F, t34545: F, t34547: F, t34549: F, t34553: F, t34557: F, t34559: F, t34562: F, t34563: F) -> F {
    let t34565 = F::new(0.18868855373762491241e-2) * t34522 + F::new(0.41930789719472202758e-3) * t34526 + t34529 / F::new(48.0) + t34532 / F::new(48.0) - t34535 + F::new(0.42874018118069736972e-3) * t30624 + t34538 - F::new(0.17149607247227894789e-2) * t34539 + F::new(0.25724410870841842183e-2) * t34541 - F::new(0.17149607247227894789e-1) * t34543 + F::new(0.51448821741683684367e-2) * t34545 - F::new(0.17149607247227894789e-2) * t34547 - F::new(0.80031500487063509014e-2) * t34549 + F::new(0.94344276868812456204e-3) * t34553 + t34557 + F::new(0.31448092289604152068e-2) * t34559 + t34562 + F::new(0.13719685797782315831e-1) * t34563;
    t34565
}
