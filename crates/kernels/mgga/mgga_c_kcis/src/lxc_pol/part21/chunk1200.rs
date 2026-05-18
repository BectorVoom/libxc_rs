//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1200/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1200<F: Float>(t2155: F, t92016: F, t91967: F, t91973: F, t91975: F, t91979: F, t91983: F, t91987: F, t91989: F, t91992: F, t91994: F, t91996: F, t91999: F, t92002: F, t92005: F, t92007: F, t92010: F, t92012: F) -> F {
    let t92017 = t2155 * t92016;
    let t92019 = F::new(0.41703125000000000001e-2) * t91967 - F::new(0.41703125000000000001e-2) * t91973 + F::new(0.83479230468750000001e-3) * t91975 - F::new(0.208515625e-2) * t91979 - F::new(0.2782641015625e-3) * t91983 + F::new(0.12985658072916666667e-2) * t91987 - F::new(0.208515625e-2) * t91989 + F::new(0.97307291666666666666e-2) * t91992 - F::new(0.2782641015625e-3) * t91994 - F::new(0.8347923046875e-3) * t91996 + F::new(0.23425829475308641975e-1) * t91999 - F::new(0.16217881944444444444e-1) * t92002 + F::new(0.48653645833333333332e-2) * t92005 + F::new(0.208515625e-2) * t92007 + F::new(0.2782641015625e-3) * t92010 + F::new(0.97307291666666666666e-2) * t92012 + F::new(0.41703125000000000001e-2) * t92017;
    t92019
}
