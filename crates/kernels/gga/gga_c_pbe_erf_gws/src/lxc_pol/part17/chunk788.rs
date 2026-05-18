//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 788/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk788<F: Float>(t1986: F, t666: F, t542: F, t671: F, t670: F, t1999: F, t245: F, t2003: F, t1984: F, t225: F, t10: F, t156: F) -> (F, F, F, F, F) {
    let t5912 = t666 * t1986;
    let t5917 = t542 * t671;
    let t5919 = F::new(0.96187034332131941129e-1) * t670 * t5917;
    let t5920 = t245 * t1999;
    let t5922 = F::new(0.33545228223331014468e-1) * t2003 * t5920;
    let t5926 = t225 * t1984;
    let t5927 = t10 * t5926;
    let t5929 = F::new(0.32463124087094530131e0) * t670 * t5927;
    let t5931 = t156 * t1999;
    (t5912, t5919, t5922, t5929, t5931)
}
