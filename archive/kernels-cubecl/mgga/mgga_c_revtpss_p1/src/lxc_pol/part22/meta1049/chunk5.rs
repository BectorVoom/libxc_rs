//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3691/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3691<F: Float>(t17209: F, t17505: F, t12855: F, t12916: F, t21120: F, t21093: F, t372: F, t13046: F, t17214: F, t21203: F, t3588: F, t3601: F, t3604: F, t3630: F, t3720: F, t44500: F, t44521: F, t56718: F, t56720: F, t56726: F, t56728: F, t56734: F, t56739: F, t56742: F, t6688: F) -> F {
    let t69812 = t17505 * t17209;
    let t69820 = t12855 * t12916 * t21120;
    let t69832 = t372 * t21093;
    let t69836 = F::cast_from(0.6351706387862183255e-3_f64) * t56718 + F::cast_from(0.30488190661738479624e-2_f64) * t21203 * t17214 + F::cast_from(0.6351706387862183255e-3_f64) * t56720 - F::cast_from(0.20325460441158986416e-2_f64) * t69812 - F::cast_from(0.19055119163586549765e-3_f64) * t56726 - F::cast_from(0.20325460441158986416e-2_f64) * t56728 - F::cast_from(0.7622047665434619906e-3_f64) * t56734 - F::cast_from(0.21172354626207277516e-3_f64) * t56739 - F::cast_from(0.1270341277572436651e-3_f64) * t56742 - F::cast_from(0.11433071498151929859e-2_f64) * t69820 - F::cast_from(0.85748036236139473944e-3_f64) * t12855 * t3720 * t6688 * t3604 * t3588 - F::cast_from(0.25724410870841842183e-2_f64) * t44500 * t3720 * t6688 * t13046 * t3601 - F::cast_from(0.57165357490759649296e-3_f64) * t44521 * t69832 * t3630;
    t69836
}
