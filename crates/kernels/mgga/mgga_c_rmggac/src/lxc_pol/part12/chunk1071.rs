//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1071/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1071<F: Float>(t3351: F, t3352: F, t511: F, t5187: F, t1668: F, t2131: F, t36902: F, t36906: F, t36910: F, t36913: F, t36916: F, t36922: F, t36925: F, t36928: F, t36936: F, t36943: F, t36948: F, t42109: F, t42114: F, t42132: F, t5355: F, t7399: F) -> F {
    let t42136 = t3351 * t3352 * t511 * t5187;
    let t42138 = -F::new(0.51077519871957407276e-4) * t42109 - F::new(0.25538759935978703638e-4) * t42114 + F::new(0.36021158228745895953e-3) * t36902 + F::new(0.72042316457491791906e-3) * t36906 + F::new(0.72042316457491791906e-3) * t36910 + F::new(0.72042316457491791906e-3) * t36913 + F::new(0.66211599834018861286e-4) * t36916 - F::new(0.38422568777328955684e-2) * t36922 - F::new(0.14408463291498358381e-2) * t36925 - F::new(0.99317399751028291929e-5) * t36928 - F::new(0.72042316457491791906e-3) * t36936 + t36943 + F::new(0.20496175532535769484e-3) * t36948 - F::new(0.4726e1) * t1668 * t7399 - F::new(0.2363e1) * t5355 * t2131 + F::new(0.72732431077987577942e-1) * t42132 - F::new(0.76616279807936110914e-4) * t42136;
    t42138
}
