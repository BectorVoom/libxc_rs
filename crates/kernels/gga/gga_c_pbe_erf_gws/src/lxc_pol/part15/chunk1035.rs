//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1035/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1035<F: Float>(t14432: F, t14477: F, t14620: F, t14660: F, t14703: F, t14739: F, t14775: F, t14814: F, t2053: F, t4188: F, t944: F, t1167: F, t810: F, t14149: F, t3324: F, t4063: F) -> (F, F, F, F, F, F, F) {
    let t14817 = t14432 + t14477 + t14620 + t14660 + t14703 + t14739 + t14775 + t14814;
    let t14821 = t4188 * t2053;
    let t14822 = t14821 * t944;
    let t14825 = t1167 * t810;
    let t14829 = t14149 * t1167;
    let t14831 = t1167 * t944;
    let t14835 = t4063 * t3324;
    (t14817, t14821, t14822, t14825, t14829, t14831, t14835)
}
