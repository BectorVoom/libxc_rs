//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1404/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1404<F: Float>(t23168: F, t6176: F, t1599: F, t1603: F, t18142: F, t18148: F, t18152: F, t18164: F, t18170: F, t18174: F, t18178: F, t18205: F, t18213: F, t23155: F, t23158: F, t23164: F, t6141: F, t6165: F) -> F {
    let t23169 = t6176 * t23168;
    let t23172 = t18142 / F::new(432.0) - t18148 + t18152 - t23155 / F::new(864.0) + F::new(11.0) / F::new(648.0) * t23158 * t1603 + t6141 * t6165 / F::new(54.0) + t23164 / F::new(1728.0) - t18164 / F::new(1296.0) - t18170 - t18174 + t18178 - t18205 + t18213 + t1599 * t23169 / F::new(96.0);
    t23172
}
