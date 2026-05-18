//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1422/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1422<F: Float>(t34772: F, t34776: F, t34797: F, t34802: F, t37072: F, t37073: F, t37074: F, t37075: F, t37076: F, t37077: F, t37080: F, t34820: F, t37082: F, t37083: F, t37084: F, t37086: F, t37087: F, t37088: F, t37089: F, t37090: F, t37091: F, t37092: F) -> (F, F) {
    let t38615 = F::new(0.19336854506021130164e-7) * t34772 - F::new(0.52389984474979915325e-9) * t34776 - t37072 - t37073 - t37074 + t37075 + t37076 + t37077 + F::new(0.29465683056794103108e-8) * t34797 - F::new(0.98332751566569010432e-8) * t34802 + t37080;
    let t38617 = t37082 - t37083 + t37084 - F::new(0.20912781366153999614e-9) * t34820 - t37086 + t37087 - t37088 + t37089 - t37090 + t37091 + t37092;
    (t38615, t38617)
}
