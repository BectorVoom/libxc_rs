//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1001/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1001<F: Float>(t1215: F, t26922: F, t26949: F, t26994: F, t29264: F, t29268: F, t29272: F, t29275: F, t29279: F, t29283: F, t29287: F, t29293: F, t29297: F, t29301: F, t29304: F, t29308: F, t5237: F, t5429: F, t5498: F, t7602: F, t7632: F, t7636: F, t7639: F, t7643: F, t7651: F) -> F {
    let t29311 = F::new(0.65854491829355115987e0) * t7602 * t5237 + F::new(0.13170898365871023197e1) * t7632 * t5429 - F::new(0.26020884564615598386e1) * t26949 * t29264 + F::new(0.17347256376410398924e1) * t26994 * t29268 + F::new(0.8673628188205199462e0) * t7651 * t29272 - F::new(0.8673628188205199462e0) * t29275 * t7639 + F::new(0.8673628188205199462e0) * t7643 * t29279 + F::new(0.8673628188205199462e0) * t7651 * t29283 + F::new(0.17347256376410398924e1) * t26994 * t29287 - F::new(0.65854491829355115987e0) * t7632 * t5498 - F::new(0.17347256376410398924e1) * t7643 * t29293 + F::new(0.17347256376410398924e1) * t7636 * t29297 - F::new(0.8673628188205199462e0) * t7636 * t29301 - F::new(0.65854491829355115987e0) * t29304 * t1215 + F::new(0.8673628188205199462e0) * t26922 * t29308;
    t29311
}
