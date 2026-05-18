//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1172/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1172<F: Float>(t20912: F, t337: F, t6560: F, t2146: F, t6535: F, t6702: F, t6258: F, t6711: F, t2293: F, t6455: F, t2262: F, t359: F, t362: F) -> (F, F, F, F, F) {
    let t20914 = t6560 * t337 * t20912;
    let t20916 = F::new(3.0) / F::new(4.0) * t2146 * t20914;
    let t20919 = t6702 * t6535 / F::new(6.0);
    let t20921 = t6711 * t6258 / F::new(8.0);
    let t20926 = t6455 * t2293;
    let t20930 = F::new(1.0) / t2262 / t359 * t362;
    (t20916, t20919, t20921, t20926, t20930)
}
