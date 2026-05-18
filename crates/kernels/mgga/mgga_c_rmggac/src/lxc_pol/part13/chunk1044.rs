//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1044/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1044<F: Float>(t38872: F, t38881: F, t38886: F, t34803: F, t34810: F, t34820: F, t37228: F, t38874: F, t38876: F, t38889: F, t38899: F, t38901: F, t38908: F, t38913: F, t38918: F, t38922: F, t38926: F) -> F {
    let t42767 = F::new(0.20496175532535769482e-3) * t38872;
    let t42771 = F::new(0.86737941314158990616e-4) * t38881;
    let t42772 = F::new(0.86737941314158990616e-4) * t38886;
    let t42783 = -t42767 - F::new(0.3842256877732895568e-2) * t38874 + F::new(0.92232789896410962669e-3) * t38876 - F::new(0.53337116123857557162e0) * t34803 + t42771 + t42772 + F::new(0.162600798888400151e-2) * t38889 + F::new(0.20455996240684006298e-1) * t38899 - t37228 - F::new(0.2727466165424534173e-1) * t38901 - F::new(0.1333427903096438929e0) * t34810 - F::new(0.36366215538993788974e-1) * t34820 - F::new(0.638468998399467591e-4) * t38908 - F::new(0.638468998399467591e-4) * t38913 - F::new(0.5107751987195740728e-4) * t38918 + F::new(0.15323255961587222184e-3) * t38922 - F::new(0.10215503974391481456e-3) * t38926;
    t42783
}
