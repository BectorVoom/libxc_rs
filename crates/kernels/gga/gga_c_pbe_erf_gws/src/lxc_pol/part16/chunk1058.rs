//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1058/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1058<F: Float>(t13893: F, t3997: F, t2238: F, t4386: F, t13808: F, t14132: F, t1176: F, t2332: F, t931: F, t3985: F, t2079: F, t376: F, t14797: F, t3973: F, t2299: F, t254: F, t3970: F) -> (F, F, F, F, F, F, F, F) {
    let t51509 = t13893 * t3997;
    let t51511 = t4386 * t2238;
    let t51526 = t13808 * t14132;
    let t51529 = t1176 * t2332 * t931;
    let t51530 = t51529 * t3985;
    let t51543 = t376 * t2079;
    let t51548 = t3973 * t14797;
    let t51555 = t3970 * t2299 * t254;
    (t51509, t51511, t51526, t51529, t51530, t51543, t51548, t51555)
}
