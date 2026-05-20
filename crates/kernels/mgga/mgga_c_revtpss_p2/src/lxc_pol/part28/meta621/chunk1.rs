//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2190/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2190<F: Float>(t2453: F, t2458: F, t7760: F, t25326: F, t25394: F, t27199: F, t887: F, t93306: F, t93312: F, t93315: F, t93318: F, t93322: F, t93324: F, t93349: F, t99412: F, t99414: F, t99420: F, t99423: F, t99425: F, t99429: F) -> F {
    let t99435 = t2453 * t7760 * t2458;
    let t99440 = F::cast_from(0.19274729307122665472e-1_f64) * t99412 + F::cast_from(0.52041769129231196772e1_f64) * t93349 * t99414 * t25394 + F::cast_from(0.34270468708064099208e-1_f64) * t93306 - t99420 + F::cast_from(0.4818682326780666368e-3_f64) * t99423 - F::cast_from(0.22849835011101738147e-2_f64) * t99425 + F::cast_from(0.8673628188205199462e0_f64) * t27199 * t25326 - F::cast_from(0.13170898365871023197e1_f64) * t99429 * t887 + F::cast_from(0.25702851531048074406e-1_f64) * t93312 + F::cast_from(0.14456046980341999104e-1_f64) * t93315 + F::cast_from(0.11565819519348392139e-2_f64) * t99435 - F::cast_from(0.77108554593144223218e-1_f64) * t93318 - F::cast_from(0.14456046980341999104e-1_f64) * t93322 + F::cast_from(0.34270468708064099208e-1_f64) * t93324;
    t99440
}
