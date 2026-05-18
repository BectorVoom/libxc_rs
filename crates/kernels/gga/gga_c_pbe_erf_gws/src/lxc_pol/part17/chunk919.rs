//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 919/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk919<F: Float>(t4821: F, t4823: F, t4830: F, t2474: F, t75: F, t472: F, t4851: F, t4853: F, t4857: F, t4860: F, t4826: F, t4837: F, t4840: F, t4843: F, t4846: F, t4849: F, t4856: F, t4864: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8026 = F::new(16.0) * t4821;
    let t8027 = F::new(4.0) * t4823;
    let t8028 = F::new(40.0) * t4830;
    let t8029 = t2474 * t75;
    let t8030 = t8029 * t472;
    let t8031 = F::new(0.11696446794910408142e1) * t8030;
    let t8032 = F::new(0.21687161765563048428e-1) * t4851;
    let t8033 = F::new(32.0) * t4853;
    let t8034 = F::new(48.0) * t4857;
    let t8035 = F::new(80.0) * t4860;
    let t8036 = -t8026 - t8027 + t4826 + t8028 - t8031 - t4837 - t4840 - t4843 + t4846 + t4849 + t8032 - t8033 - t4856 + t8034 + t8035 - t4864;
    (t8026, t8027, t8028, t8031, t8032, t8033, t8034, t8035, t8036)
}
