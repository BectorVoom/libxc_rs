//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3691/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3691(t17209: f64, t17505: f64, t12855: f64, t12916: f64, t21120: f64, t21093: f64, t372: f64, t13046: f64, t17214: f64, t21203: f64, t3588: f64, t3601: f64, t3604: f64, t3630: f64, t3720: f64, t44500: f64, t44521: f64, t56718: f64, t56720: f64, t56726: f64, t56728: f64, t56734: f64, t56739: f64, t56742: f64, t6688: f64) -> f64 {
    let t69812 = t17505 * t17209;
    let t69820 = t12855 * t12916 * t21120;
    let t69832 = t372 * t21093;
    let t69836 = 0.6351706387862183255e-3_f64 * t56718 + 0.30488190661738479624e-2_f64 * t21203 * t17214 + 0.6351706387862183255e-3_f64 * t56720 - 0.20325460441158986416e-2_f64 * t69812 - 0.19055119163586549765e-3_f64 * t56726 - 0.20325460441158986416e-2_f64 * t56728 - 0.7622047665434619906e-3_f64 * t56734 - 0.21172354626207277516e-3_f64 * t56739 - 0.1270341277572436651e-3_f64 * t56742 - 0.11433071498151929859e-2_f64 * t69820 - 0.85748036236139473944e-3_f64 * t12855 * t3720 * t6688 * t3604 * t3588 - 0.25724410870841842183e-2_f64 * t44500 * t3720 * t6688 * t13046 * t3601 - 0.57165357490759649296e-3_f64 * t44521 * t69832 * t3630;
    t69836
}
