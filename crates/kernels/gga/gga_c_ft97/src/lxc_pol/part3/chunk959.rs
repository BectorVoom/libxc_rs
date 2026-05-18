//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 959/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk959<F: Float>(t1091: F, t2923: F, t4370: F, t2253: F, t5470: F, t5459: F, t10304: F, t4939: F, t2697: F, t4977: F, t18127: F, t801: F) -> (F, F, F, F, F, F) {
    let t18820 = t2923 * t1091 * t4370;
    let t18823 = t2253 * t5470;
    let t18825 = t2253 * t5459;
    let t18826 = t10304 * t4939;
    let t18831 = t2697 * t4977;
    let t18834 = t801 * t18127;
    (t18820, t18823, t18825, t18826, t18831, t18834)
}
