//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1089/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1089<F: Float>(t36533: F, t695: F, t7639: F, t26477: F, t7642: F, t209: F, t213: F, t36902: F, t8762: F, t2155: F, t91967: F, t91973: F, t91975: F, t91979: F, t91983: F, t91987: F, t91989: F, t91992: F, t91994: F, t91996: F, t91999: F, t92002: F, t92005: F, t92007: F) -> (F, F) {
    let t92010 = t36533 * t695 * t7639;
    let t92012 = t7642 * t26477;
    let t92016 = t209 * t213 * t36902 * t8762;
    let t92017 = t2155 * t92016;
    let t92019 = 0.41703125000000000001e-2 * t91967 - 0.41703125000000000001e-2 * t91973 + 0.83479230468750000001e-3 * t91975 - 0.208515625e-2 * t91979 - 0.2782641015625e-3 * t91983 + 0.12985658072916666667e-2 * t91987 - 0.208515625e-2 * t91989 + 0.97307291666666666666e-2 * t91992 - 0.2782641015625e-3 * t91994 - 0.8347923046875e-3 * t91996 + 0.23425829475308641975e-1 * t91999 - 0.16217881944444444444e-1 * t92002 + 0.48653645833333333332e-2 * t92005 + 0.208515625e-2 * t92007 + 0.2782641015625e-3 * t92010 + 0.97307291666666666666e-2 * t92012 + 0.41703125000000000001e-2 * t92017;
    (t92016, t92019)
}
