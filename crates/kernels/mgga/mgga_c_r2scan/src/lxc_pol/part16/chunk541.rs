//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 541/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk541<F: Float>(t322: F, t1300: F, t2941: F, t2944: F, t327: F, t833: F, t834: F, t330: F, t1018: F, t2940: F) -> (F, F, F, F, F) {
    let t332 = F::new(0.25e1) < t322;
    let t2951 = -F::new(0.64e0) * t2941 * t327 - F::new(0.128e1) * t2944 * t833 - F::new(0.128e1) * t1300 * t2944 - F::new(0.64e0) * t834 * t2941;
    let t2952 = t2951 * t330;
    let t2953 = t1018 * t1018;
    let t2954 = t2953 * t330;
    let t2956 = piecewise3::<F>(t332, F::new(0.0), t2940);
    (t2951, t2952, t2953, t2954, t2956)
}
