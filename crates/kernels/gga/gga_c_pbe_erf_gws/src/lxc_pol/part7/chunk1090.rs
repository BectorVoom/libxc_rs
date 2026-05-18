//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1090/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1090<F: Float>(t18709: F, t18838: F, t18850: F, t18853: F, t18863: F, t18914: F, t18916: F, t18920: F, t18924: F, t18928: F, t18933: F, t18935: F, t18939: F, t19517: F, t19521: F, t19525: F) -> F {
    let t19526 = t18709 + t18914 - t18838 + t18916 - t19517 + t18850 + t18920 + t18924 + t18853 - t19521 - t18863 + t19525 + t18928 - t18933 + t18935 + t18939;
    t19526
}
