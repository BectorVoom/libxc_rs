//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1088/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1088<F: Float>(t18645: F, t18647: F, t18655: F, t18658: F, t18660: F, t18662: F, t18667: F, t18669: F, t18701: F, t18703: F, t18705: F, t18707: F, t19502: F, t19504: F, t19505: F, t2053: F, t2429: F, t321: F, t382: F, t6837: F, t944: F) -> F {
    let t19513 = -F::new(4.0) * t2053 * t321 * t6837 * t944 + F::new(18.0) * t19505 * t2429 * t382 - t18645 - t18647 + t18655 + t18658 - t18660 + t18662 - t18667 - t18669 + t18701 - t18703 + t18705 + t18707 + t19502 + t19504;
    t19513
}
