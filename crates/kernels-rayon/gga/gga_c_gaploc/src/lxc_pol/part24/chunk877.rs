//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 877/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk877(t4820: f64, t8756: f64, t2023: f64, t2028: f64, t2043: f64, t2194: f64, t2197: f64, t2201: f64, t2989: f64, t2995: f64, t3002: f64, t3035: f64, t3040: f64, t3050: f64, t3067: f64, t5598: f64, t5629: f64, t5662: f64, t5715: f64, t5983: f64, t6096: f64, t7716: f64, t784: f64, t797: f64, t807: f64, t813: f64, t833: f64, t8693: f64, t8696: f64, t8722: f64, t8726: f64, t8730: f64, t8733: f64, t8738: f64, t8741: f64, t8749: f64, t8753: f64) -> f64 {
    let t8757 = t4820 * t8756;
    let t8760 = 0.47667319935800568892e0_f64 * t3050 * t784 - 0.61348681526273199482e1_f64 * t2194 * t2995 - 0.61348681526273199482e1_f64 * t813 * t8693 - 0.47667319935800568892e0_f64 * t797 * t8696 + 0.61348681526273199482e1_f64 * t2197 * t2989 + 0.29792074959875355558e-1_f64 * t7716 + 0.46011511144704899612e1_f64 * t2197 * t3067 + 0.23005755572352449806e1_f64 * t833 * t8722 + 0.46011511144704899612e1_f64 * t5629 * t8726 - 0.46011511144704899612e1_f64 * t2201 * t8730 + 0.61348681526273199482e1_f64 * t807 * t8733 + 0.35750489951850426669e0_f64 * t2043 * t3040 - 0.71500979903700853338e0_f64 * t5983 * t8738 + 0.71500979903700853338e0_f64 * t8741 * t6096 - 0.47667319935800568892e0_f64 * t3035 * t5715 - 0.51123901271894332905e0_f64 * t5662 * t3002 + 0.79445533226334281486e-1_f64 * t8749 * t2023 - 0.79445533226334281486e-1_f64 * t8753 * t2028 - 0.79445533226334281486e-1_f64 * t5598 * t8757;
    t8760
}
