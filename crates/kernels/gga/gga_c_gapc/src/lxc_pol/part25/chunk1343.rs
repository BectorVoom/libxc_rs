//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1343/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1343<F: Float>(t35848: F, t35851: F, t35853: F, t35858: F, t35861: F, t35865: F, t35867: F, t35869: F, t35871: F, t35875: F, t35878: F, t35881: F, t35883: F, t35885: F) -> F {
    let t36188 = F::cast_from(0.10943177049050145945e-4_f64) * t35848 - F::cast_from(0.29357452990051769742e-5_f64) * t35851 + F::cast_from(0.46971924784082831588e-5_f64) * t35853 - F::cast_from(0.23485962392041415794e-4_f64) * t35858 + F::cast_from(0.68394856556563412152e-6_f64) * t35861 + F::cast_from(0.19948499828997661878e-6_f64) * t35865 - F::cast_from(0.13272015205919237571e-4_f64) * t35867 + F::cast_from(0.18788769913633132635e-3_f64) * t35869 + F::cast_from(0.18788769913633132635e-3_f64) * t35871 - F::cast_from(0.61555370900907070936e-5_f64) * t35875 + F::cast_from(0.11416787273909021566e-5_f64) * t35878 - F::cast_from(0.67250428176206296283e-7_f64) * t35881 - F::cast_from(0.23485962392041415794e-4_f64) * t35883 - F::cast_from(0.46971924784082831588e-4_f64) * t35885;
    t36188
}
