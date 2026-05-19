//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 582/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk582<F: Float>(t3831: F, t7877: F, t1354: F, t7897: F, t1398: F, t7740: F, t1375: F, t7744: F, t1349: F, t1391: F, t158: F, t173: F, t3844: F, t3848: F, t3851: F, t3852: F, t3858: F, t5802: F, t5804: F, t7710: F) -> (F, F, F, F, F) {
    let t8108 = t3831 * t7877;
    let t8111 = t1354 * t7897;
    let t8123 = t1398 * t7740;
    let t8126 = t1375 * t7744;
    let t8129 = -t3844 - t3848 + t3851 - t3852 + t3858 + F::cast_from(0.11955719325063177623e-1_f64) * t1349 * t7710 - F::cast_from(0.5179538907796306876e-4_f64) * t1391 * t7710 - F::cast_from(0.23911438650126355246e-1_f64) * t5802 + F::cast_from(0.20718155631185227504e-3_f64) * t5804 - F::new(0.10082625e-4) * t173 * t8123 - F::new(0.3513e-2) * t158 * t8126;
    (t8108, t8111, t8123, t8126, t8129)
}
