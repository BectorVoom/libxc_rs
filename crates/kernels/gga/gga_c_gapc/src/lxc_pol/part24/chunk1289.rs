//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1289/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1289<F: Float>(t35848: F, t35851: F, t35853: F, t35858: F, t35861: F, t35865: F, t35867: F, t35869: F, t35871: F, t35875: F, t35878: F, t35881: F, t35883: F, t35885: F) -> F {
    let t37554 = F::new(0.2188635409810029189e-4) * t35848 - F::new(0.58714905980103539484e-5) * t35851 + F::new(0.93943849568165663176e-5) * t35853 - F::new(0.46971924784082831588e-4) * t35858 + F::new(0.13678971311312682431e-5) * t35861 + F::new(0.39896999657995323756e-6) * t35865 - F::new(0.26544030411838475142e-4) * t35867 + F::new(0.3757753982726626527e-3) * t35869 + F::new(0.3757753982726626527e-3) * t35871 - F::new(0.12311074180181414188e-4) * t35875 + F::new(0.22833574547818043134e-5) * t35878 - F::new(0.13450085635241259256e-6) * t35881 - F::new(0.46971924784082831588e-4) * t35883 - F::new(0.93943849568165663176e-4) * t35885;
    t37554
}
