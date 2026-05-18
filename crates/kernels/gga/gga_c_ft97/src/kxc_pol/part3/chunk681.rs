//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 681/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk681<F: Float>(t3312: F, t3682: F, t4026: F, t4399: F, t1851: F, t971: F, t1882: F, t3010: F, t2989: F, t2994: F, t2985: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10947 = F::new(2.0) * t3312;
    let t10948 = F::new(2.0) * t3682;
    let t10949 = F::new(2.0) * t4026;
    let t10950 = F::new(2.0) * t4399;
    let t10969 = t971 * t1851;
    let t10992 = t1882 * t3010;
    let t10993 = t10992 / F::new(27.0);
    let t11021 = t1882 * t2989;
    let t11022 = t11021 / F::new(27.0);
    let t11023 = t1882 * t2994;
    let t11024 = F::new(2.0) / F::new(27.0) * t11023;
    let t11025 = t1882 * t2985;
    (t10947, t10948, t10949, t10950, t10969, t10992, t10993, t11021, t11022, t11023, t11024, t11025)
}
