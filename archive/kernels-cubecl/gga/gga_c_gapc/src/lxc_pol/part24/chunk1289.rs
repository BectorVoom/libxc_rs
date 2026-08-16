//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1289/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1289<F: Float>(t35848: F, t35851: F, t35853: F, t35858: F, t35861: F, t35865: F, t35867: F, t35869: F, t35871: F, t35875: F, t35878: F, t35881: F, t35883: F, t35885: F) -> F {
    let t37554 = F::cast_from(0.2188635409810029189e-4_f64) * t35848 - F::cast_from(0.58714905980103539484e-5_f64) * t35851 + F::cast_from(0.93943849568165663176e-5_f64) * t35853 - F::cast_from(0.46971924784082831588e-4_f64) * t35858 + F::cast_from(0.13678971311312682431e-5_f64) * t35861 + F::cast_from(0.39896999657995323756e-6_f64) * t35865 - F::cast_from(0.26544030411838475142e-4_f64) * t35867 + F::cast_from(0.3757753982726626527e-3_f64) * t35869 + F::cast_from(0.3757753982726626527e-3_f64) * t35871 - F::cast_from(0.12311074180181414188e-4_f64) * t35875 + F::cast_from(0.22833574547818043134e-5_f64) * t35878 - F::cast_from(0.13450085635241259256e-6_f64) * t35881 - F::cast_from(0.46971924784082831588e-4_f64) * t35883 - F::cast_from(0.93943849568165663176e-4_f64) * t35885;
    t37554
}
