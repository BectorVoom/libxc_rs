//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1068/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1068<F: Float>(t10896: F, t7025: F, t25282: F, t9802: F, t92942: F, t92944: F, t92946: F, t92948: F, t92952: F, t92956: F, t92958: F, t92960: F, t92963: F, t92966: F, t92969: F, t92971: F) -> (F,) {
    let t92973 = t7025 * t10896;
    let t92975 = t9802 * t25282;
    let t92976 = 0.91476005056713590805e-4 * t92975;
    let t92977 = 0.25724410870841842183e-2 * t92942 - 0.25724410870841842184e-1 * t92944 + 0.51448821741683684367e-2 * t92946 + 0.51448821741683684367e-2 * t92948 - 0.48018900292238105409e-1 * t92952 + 0.6098400337114239387e-3 * t92956 + 0.51448821741683684367e-2 * t92958 - 0.12862205435420921092e-2 * t92960 + 0.15246000842785598468e-4 * t92963 - 0.1084295579938911763e-3 * t92966 - 35.0 / 72.0 * t92969 + 7.0 / 48.0 * t92971 - t92973 / 48.0 + t92976;
    (t92977,)
}
