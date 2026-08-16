//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1071/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1071(t3351: f64, t3352: f64, t511: f64, t5187: f64, t1668: f64, t2131: f64, t36902: f64, t36906: f64, t36910: f64, t36913: f64, t36916: f64, t36922: f64, t36925: f64, t36928: f64, t36936: f64, t36943: f64, t36948: f64, t42109: f64, t42114: f64, t42132: f64, t5355: f64, t7399: f64) -> f64 {
    let t42136 = t3351 * t3352 * t511 * t5187;
    let t42138 = -0.51077519871957407276e-4_f64 * t42109 - 0.25538759935978703638e-4_f64 * t42114 + 0.36021158228745895953e-3_f64 * t36902 + 0.72042316457491791906e-3_f64 * t36906 + 0.72042316457491791906e-3_f64 * t36910 + 0.72042316457491791906e-3_f64 * t36913 + 0.66211599834018861286e-4_f64 * t36916 - 0.38422568777328955684e-2_f64 * t36922 - 0.14408463291498358381e-2_f64 * t36925 - 0.99317399751028291929e-5_f64 * t36928 - 0.72042316457491791906e-3_f64 * t36936 + t36943 + 0.20496175532535769484e-3_f64 * t36948 - 0.4726e1_f64 * t1668 * t7399 - 0.2363e1_f64 * t5355 * t2131 + 0.72732431077987577942e-1_f64 * t42132 - 0.76616279807936110914e-4_f64 * t42136;
    t42138
}
