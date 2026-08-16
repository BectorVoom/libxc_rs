//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1197/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1197<F: Float>(t2080: F, t2084: F, t51502: F, t833: F, t13800: F, t13972: F, t13893: F, t3997: F, t2238: F, t4386: F, t13808: F, t14132: F) -> (F, F, F, F, F) {
    let t51505 = t2080 * t2084 * t51502 * t833;
    let t51507 = t13972 * t13800;
    let t51509 = t13893 * t3997;
    let t51511 = t4386 * t2238;
    let t51526 = t13808 * t14132;
    (t51505, t51507, t51509, t51511, t51526)
}
