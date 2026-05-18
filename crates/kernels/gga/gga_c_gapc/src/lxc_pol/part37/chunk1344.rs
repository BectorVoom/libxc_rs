//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1344/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1344<F: Float>(t35848: F, t35851: F, t35853: F, t35858: F, t35861: F, t35865: F, t35867: F, t35869: F, t35871: F, t35875: F, t35878: F, t35881: F, t35883: F, t35885: F) -> F {
    let t36188 = F::new(0.10943177049050145945e-4) * t35848 - F::new(0.29357452990051769742e-5) * t35851 + F::new(0.46971924784082831588e-5) * t35853 - F::new(0.23485962392041415794e-4) * t35858 + F::new(0.68394856556563412152e-6) * t35861 + F::new(0.19948499828997661878e-6) * t35865 - F::new(0.13272015205919237571e-4) * t35867 + F::new(0.18788769913633132635e-3) * t35869 + F::new(0.18788769913633132635e-3) * t35871 - F::new(0.61555370900907070936e-5) * t35875 + F::new(0.11416787273909021566e-5) * t35878 - F::new(0.67250428176206296283e-7) * t35881 - F::new(0.23485962392041415794e-4) * t35883 - F::new(0.46971924784082831588e-4) * t35885;
    t36188
}
