//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1951/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1951<F: Float>(t30: F, t1469: F, t1996: F, t29726: F, t29931: F, t45: F, t5825: F, t7856: F, t33: F, t5966: F, t1963: F, t25759: F, t29598: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t29938 = piecewise3::<F>(t120, t29726, t29931 * t45 / F::new(2.0) + t7856 * t1469 + t1996 * t5825 / F::new(2.0));
    let t29939 = t33 * t5966;
    let t29940 = t1963 * t29939;
    let t29946 = t25759 * t29598;
    (t29938, t29939, t29940, t29946)
}
