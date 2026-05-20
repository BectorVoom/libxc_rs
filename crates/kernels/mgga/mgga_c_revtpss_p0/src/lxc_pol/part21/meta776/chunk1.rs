//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2767/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2767<F: Float>(t50868: F, t14325: F, t14622: F, t40156: F, t14440: F, t2398: F, t40172: F, t40178: F, t14369: F, t2258: F, t4401: F, t14370: F) -> (F, F, F, F, F, F, F, F) {
    let t50869 = F::new(72.0) * t50868;
    let t50871 = F::new(36.0) * t14325 * t14622;
    let t50872 = F::cast_from(0.51947577317044391277e2_f64) * t40156;
    let t50873 = t2398 * t14440;
    let t50874 = F::new(12.0) * t50873;
    let t50875 = F::cast_from(0.30762056574649219973e4_f64) * t40172;
    let t50876 = F::new(36.0) * t40178;
    let t50878 = t4401 * t14369 * t2258;
    let t50879 = F::new(36.0) * t50878;
    let t50880 = t14325 * t14370;
    (t50869, t50871, t50872, t50874, t50875, t50876, t50879, t50880)
}
