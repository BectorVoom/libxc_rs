//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1178/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1178(t34698: f64, t34702: f64, t34704: f64, t34710: f64, t34712: f64, t34722: f64, t34724: f64, t34738: f64, t32557: f64, t34694: f64, t34700: f64, t34708: f64, t34716: f64, t34718: f64, t34728: f64, t34732: f64, t34736: f64, t34740: f64) -> f64 {
    let t37211 = 0.42874018118069736972e-2_f64 * t34698;
    let t37213 = 0.25724410870841842184e-1_f64 * t34702;
    let t37214 = 0.1543464652250510531e-1_f64 * t34704;
    let t37216 = 0.25724410870841842184e-2_f64 * t34710;
    let t37217 = 0.25724410870841842184e-2_f64 * t34712;
    let t37220 = 0.31448092289604152068e-2_f64 * t34722;
    let t37221 = 0.18868855373762491241e-1_f64 * t34724;
    let t37225 = 0.25724410870841842184e-2_f64 * t34738;
    let t37227 = 0.2750625e0_f64 * t34694 - t37211 + 0.34299214494455789578e-2_f64 * t34700 - t37213 - t37214 + 0.21437009059034868486e-2_f64 * t34708 + t37216 + t37217 - 0.51448821741683684368e-2_f64 * t34716 - 0.51448821741683684366e-2_f64 * t34718 + t37220 + t37221 + 0.28303283060643736862e-1_f64 * t34728 - 0.18868855373762491241e-1_f64 * t34732 - 0.12862205435420921092e-1_f64 * t34736 + t37225 + 0.17149607247227894789e-2_f64 * t34740 - t32557;
    t37227
}
