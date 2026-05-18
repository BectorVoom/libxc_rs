//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 730/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk730<F: Float>(t1628: F, t2427: F, t1417: F, t1424: F, t1458: F, t193: F, t2385: F, t2457: F, t4425: F, t4819: F, t557: F, t574: F, t597: F, t6817: F, t6820: F, t6824: F, t6825: F, t6831: F, t6835: F, t6838: F, t6841: F, t6845: F, t6847: F, t6849: F, t6852: F, t6856: F, t6860: F, t6863: F, t6866: F, t6869: F) -> F {
    let t6872 = t1628 * t2427;
    let t6875 = F::new(0.11502877786176224903e2) * t597 * t6817 - F::new(0.10725146985555128001e1) * t2385 * t6820 - F::new(0.15889106645266856297e0) * t6824 * t6825 - F::new(0.51123901271894332905e0) * t4425 * t2457 + F::new(0.79445533226334281486e-1) * t6831 * t1417 - F::new(0.79445533226334281486e-1) * t6835 * t1424 - F::new(0.79445533226334281486e-1) * t4819 * t6838 + F::new(0.14896037479937677779e-1) * t6841 + F::new(0.29792074959875355558e-1) * t6845 - F::new(0.95857314884801874192e-1) * t6847 + F::new(0.42603251059911944086e-1) * t6849 + F::new(0.21450293971110256002e1) * t6852 * t1458 - F::new(0.13491029502305448961e0) * t6856 + F::new(0.29792074959875355558e-1) * t6860 + F::new(0.23005755572352449806e1) * t597 * t6863 + F::new(0.35750489951850426669e0) * t6866 * t193 - F::new(0.47667319935800568892e0) * t557 * t6869 - F::new(0.61348681526273199482e1) * t574 * t6872;
    t6875
}
