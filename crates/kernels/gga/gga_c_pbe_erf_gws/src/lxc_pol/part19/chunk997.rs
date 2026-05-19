//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 997/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk997<F: Float>(t10845: F, t10847: F, t10850: F, t10852: F, t10856: F, t10859: F, t10863: F, t10866: F, t10870: F, t10873: F, t10875: F, t5945: F, t5948: F, t5952: F, t5954: F, t7672: F, t7715: F) -> F {
    let t11219 = t7672 + F::new(8.0) / F::new(3.0) * t5945 + t5948 + t5952 + t10845 - t10847 + t10850 + t10852 - t10856 - t10859 + t10863 + t10866 - t7715 + F::cast_from(0.11181742741110338156e-1_f64) * t5954 - t10870 - t10873 - t10875;
    t11219
}
