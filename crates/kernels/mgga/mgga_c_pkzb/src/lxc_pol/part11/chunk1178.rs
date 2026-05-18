//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1178/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1178<F: Float>(t24604: F, t24606: F, t16593: F, t16595: F, t16592: F, t28954: F, t28955: F, t28956: F, t28957: F, t28958: F, t28959: F, t16600: F) -> (F, F, F, F, F, F) {
    let t28960 = F::new(0.32530743900905219526e-1) * t24604;
    let t28961 = F::new(0.35089341735807877242e1) * t24606;
    let t28962 = F::new(0.35089341735807877242e1) * t16593;
    let t28963 = F::new(0.21687162600603479684e-1) * t16595;
    let t28964 = t28954 - t28955 - t28956 - t28957 - t28958 + t28959 + t28960 + t28961 - t16592 - t28962 - t28963;
    let t28966 = F::new(0.32530743900905219526e-1) * t16600;
    (t28960, t28961, t28962, t28963, t28964, t28966)
}
