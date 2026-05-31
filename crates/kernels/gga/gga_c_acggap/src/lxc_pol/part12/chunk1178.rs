//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1178/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1178<F: Float>(t34698: F, t34702: F, t34704: F, t34710: F, t34712: F, t34722: F, t34724: F, t34738: F, t32557: F, t34694: F, t34700: F, t34708: F, t34716: F, t34718: F, t34728: F, t34732: F, t34736: F, t34740: F) -> F {
    let t37211 = F::cast_from(0.42874018118069736972e-2_f64) * t34698;
    let t37213 = F::cast_from(0.25724410870841842184e-1_f64) * t34702;
    let t37214 = F::cast_from(0.1543464652250510531e-1_f64) * t34704;
    let t37216 = F::cast_from(0.25724410870841842184e-2_f64) * t34710;
    let t37217 = F::cast_from(0.25724410870841842184e-2_f64) * t34712;
    let t37220 = F::cast_from(0.31448092289604152068e-2_f64) * t34722;
    let t37221 = F::cast_from(0.18868855373762491241e-1_f64) * t34724;
    let t37225 = F::cast_from(0.25724410870841842184e-2_f64) * t34738;
    let t37227 = F::cast_from(0.2750625e0_f64) * t34694 - t37211 + F::cast_from(0.34299214494455789578e-2_f64) * t34700 - t37213 - t37214 + F::cast_from(0.21437009059034868486e-2_f64) * t34708 + t37216 + t37217 - F::cast_from(0.51448821741683684368e-2_f64) * t34716 - F::cast_from(0.51448821741683684366e-2_f64) * t34718 + t37220 + t37221 + F::cast_from(0.28303283060643736862e-1_f64) * t34728 - F::cast_from(0.18868855373762491241e-1_f64) * t34732 - F::cast_from(0.12862205435420921092e-1_f64) * t34736 + t37225 + F::cast_from(0.17149607247227894789e-2_f64) * t34740 - t32557;
    t37227
}
