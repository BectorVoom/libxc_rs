//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 876/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk876<F: Float>(t4820: F, t8756: F, t2023: F, t2028: F, t2043: F, t2194: F, t2197: F, t2201: F, t2989: F, t2995: F, t3002: F, t3035: F, t3040: F, t3050: F, t3067: F, t5598: F, t5629: F, t5662: F, t5715: F, t5983: F, t6096: F, t7716: F, t784: F, t797: F, t807: F, t813: F, t833: F, t8693: F, t8696: F, t8722: F, t8726: F, t8730: F, t8733: F, t8738: F, t8741: F, t8749: F, t8753: F) -> F {
    let t8757 = t4820 * t8756;
    let t8760 = F::cast_from(0.47667319935800568892e0_f64) * t3050 * t784 - F::cast_from(0.61348681526273199482e1_f64) * t2194 * t2995 - F::cast_from(0.61348681526273199482e1_f64) * t813 * t8693 - F::cast_from(0.47667319935800568892e0_f64) * t797 * t8696 + F::cast_from(0.61348681526273199482e1_f64) * t2197 * t2989 + F::cast_from(0.29792074959875355558e-1_f64) * t7716 + F::cast_from(0.46011511144704899612e1_f64) * t2197 * t3067 + F::cast_from(0.23005755572352449806e1_f64) * t833 * t8722 + F::cast_from(0.46011511144704899612e1_f64) * t5629 * t8726 - F::cast_from(0.46011511144704899612e1_f64) * t2201 * t8730 + F::cast_from(0.61348681526273199482e1_f64) * t807 * t8733 + F::cast_from(0.35750489951850426669e0_f64) * t2043 * t3040 - F::cast_from(0.71500979903700853338e0_f64) * t5983 * t8738 + F::cast_from(0.71500979903700853338e0_f64) * t8741 * t6096 - F::cast_from(0.47667319935800568892e0_f64) * t3035 * t5715 - F::cast_from(0.51123901271894332905e0_f64) * t5662 * t3002 + F::cast_from(0.79445533226334281486e-1_f64) * t8749 * t2023 - F::cast_from(0.79445533226334281486e-1_f64) * t8753 * t2028 - F::cast_from(0.79445533226334281486e-1_f64) * t5598 * t8757;
    t8760
}
