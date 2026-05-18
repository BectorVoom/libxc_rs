//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 948/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk948<F: Float>(t9816: F, t9818: F, t9820: F, t9822: F, t9824: F, t9828: F, t9830: F, t9833: F, t9836: F, t9839: F, t9847: F, t9850: F, t9853: F) -> F {
    let t10961 = -F::new(0.11594181388521408694e-4) * t9816 + F::new(0.57970906942607043472e-5) * t9818 - F::new(0.24326659074064819792e-2) * t9820 - F::new(0.12974218172834570556e-1) * t9822 - F::new(0.12974218172834570556e-1) * t9824 + F::new(0.28985453471303521736e-5) * t9828 - F::new(0.15458908518028544927e-5) * t9830 + F::new(0.2748593934505475288e-5) * t9833 + F::new(0.34752370105806885418e-3) * t9836 + F::new(0.51491428373437201896e-5) * t9839 - F::new(0.45839761994185933919e-8) * t9847 - F::new(0.42270452978984302532e-6) * t9850 - F::new(0.24760339692676868218e-5) * t9853;
    t10961
}
