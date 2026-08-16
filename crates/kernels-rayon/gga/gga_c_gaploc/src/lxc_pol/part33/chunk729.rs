//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 729/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk729(t1628: f64, t2427: f64, t1417: f64, t1424: f64, t1458: f64, t193: f64, t2385: f64, t2457: f64, t4425: f64, t4819: f64, t557: f64, t574: f64, t597: f64, t6817: f64, t6820: f64, t6824: f64, t6825: f64, t6831: f64, t6835: f64, t6838: f64, t6841: f64, t6845: f64, t6847: f64, t6849: f64, t6852: f64, t6856: f64, t6860: f64, t6863: f64, t6866: f64, t6869: f64) -> f64 {
    let t6872 = t1628 * t2427;
    let t6875 = 0.11502877786176224903e2_f64 * t597 * t6817 - 0.10725146985555128001e1_f64 * t2385 * t6820 - 0.15889106645266856297e0_f64 * t6824 * t6825 - 0.51123901271894332905e0_f64 * t4425 * t2457 + 0.79445533226334281486e-1_f64 * t6831 * t1417 - 0.79445533226334281486e-1_f64 * t6835 * t1424 - 0.79445533226334281486e-1_f64 * t4819 * t6838 + 0.14896037479937677779e-1_f64 * t6841 + 0.29792074959875355558e-1_f64 * t6845 - 0.95857314884801874192e-1_f64 * t6847 + 0.42603251059911944086e-1_f64 * t6849 + 0.21450293971110256002e1_f64 * t6852 * t1458 - 0.13491029502305448961e0_f64 * t6856 + 0.29792074959875355558e-1_f64 * t6860 + 0.23005755572352449806e1_f64 * t597 * t6863 + 0.35750489951850426669e0_f64 * t6866 * t193 - 0.47667319935800568892e0_f64 * t557 * t6869 - 0.61348681526273199482e1_f64 * t574 * t6872;
    t6875
}
