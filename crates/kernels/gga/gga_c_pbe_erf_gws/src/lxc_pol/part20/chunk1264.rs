//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1264/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1264<F: Float>(t13939: F, t3083: F, t13953: F, t3070: F, t1192: F, t26654: F, t829: F, t830: F, t13808: F, t14584: F, t4130: F, t51650: F) -> (F, F, F, F, F) {
    let t54667 = F::new(7.0) / F::new(144.0) * t3083 * t13939;
    let t54681 = t13953 * t3070;
    let t54682 = F::new(7.0) / F::new(72.0) * t54681;
    let t54709 = t26654 * t1192;
    let t54711 = t829 * t830 * t54709;
    let t54716 = t13808 * t14584;
    let t54717 = F::new(7.0) / F::new(1152.0) * t54716;
    let t54719 = t51650 * t4130;
    (t54667, t54682, t54711, t54717, t54719)
}
