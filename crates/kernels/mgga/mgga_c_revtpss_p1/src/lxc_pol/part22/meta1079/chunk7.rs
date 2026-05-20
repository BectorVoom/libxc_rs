//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3877/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3877<F: Float>(t22081: F, t9962: F, t22276: F, t3989: F, t22281: F, t22056: F, t9765: F, t48865: F, t48868: F, t48872: F, t48876: F, t48879: F, t48881: F, t48888: F) -> F {
    let t74498 = t9962 * t22081;
    let t74505 = t3989 * t22276;
    let t74507 = t3989 * t22281;
    let t74511 = t9765 * t22056;
    let t74513 = -F::cast_from(0.80031500487063509015e-2_f64) * t74498 - F::cast_from(0.11433071498151929859e-3_f64) * t48865 + F::cast_from(0.10164000561857065645e-4_f64) * t48868 + F::cast_from(0.36143185997963725434e-3_f64) * t48872 - F::cast_from(0.20328001123714131289e-4_f64) * t48876 + F::cast_from(0.16264433699083676445e-3_f64) * t48879 + F::cast_from(0.24009450146119052704e0_f64) * t74505 - F::cast_from(0.80031500487063509015e-1_f64) * t74507 + F::cast_from(0.60976381323476959249e-3_f64) * t48881 + F::cast_from(0.24009450146119052705e0_f64) * t48888 + F::cast_from(0.54208002996571016773e-3_f64) * t74511;
    t74513
}
