//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1309/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1309<F: Float>(t4083: F, t8669: F, t4110: F, t8589: F, t829: F, t830: F, t52991: F, t53011: F, t53015: F, t14182: F, t26958: F, t11375: F, t14193: F, t14918: F, t22142: F, t22343: F, t2384: F, t52179: F, t52381: F, t52994: F, t52997: F, t53009: F, t53019: F, t827: F, t8793: F) -> F {
    let t54915 = F::new(7.0) / F::new(144.0) * t8669 * t4083;
    let t54916 = t8589 * t4110;
    let t54918 = t829 * t830 * t54916;
    let t54923 = F::new(7.0) / F::new(72.0) * t52991;
    let t54927 = F::new(7.0) / F::new(1152.0) * t53011;
    let t54928 = F::new(35.0) / F::new(216.0) * t53015;
    let t54937 = F::new(7.0) / F::new(72.0) * t26958 * t14182;
    let t54940 = t54915 - t827 * t54918 / F::new(48.0) - t2384 * t14918 / F::new(96.0) + t54923 - t52994 / F::new(12.0) - t52997 / F::new(12.0) - t53009 / F::new(768.0) - t54927 + t54928 + t53019 / F::new(768.0) - t22142 * t4083 / F::new(96.0) + t22343 * t14193 / F::new(48.0) - t8793 * t52381 / F::new(16.0) - t54937 - t11375 * t52179 / F::new(48.0);
    t54940
}
