//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1041/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1041<F: Float>(t10956: F, t1679: F, t467: F, t9099: F, t33857: F, t33861: F, t33867: F, t33869: F, t33874: F, t33894: F, t33960: F, t33984: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t36617 = F::new(2.0) * t1679 * t10956 * t467;
    let t36619 = F::new(4.0) * t1679 * t9099;
    let t36823 = F::cast_from(0.12579236915841660827e-2_f64) * t33857;
    let t36825 = F::new(35.0) / F::new(216.0) * t33861;
    let t36828 = F::cast_from(0.85748036236139473944e-3_f64) * t33867;
    let t36829 = F::cast_from(0.15724046144802076034e-2_f64) * t33869;
    let t36833 = F::cast_from(0.10718504529517434243e-2_f64) * t33874;
    let t36838 = F::cast_from(0.28582678745379824648e-3_f64) * t33894;
    let t36876 = F::new(0.7640625e-2) * t33960;
    let t36889 = F::cast_from(0.37737710747524982482e-2_f64) * t33984;
    (t36617, t36619, t36823, t36825, t36828, t36829, t36833, t36838, t36876, t36889)
}
