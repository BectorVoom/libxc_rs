//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1302/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1302<F: Float>(t34278: F, t1415: F, t7030: F, t8330: F, t2365: F, t25740: F, t7025: F, t26126: F, t544: F, t21139: F, t10513: F, t18067: F, t6964: F) -> (F, F, F, F, F) {
    let t34279 = F::cast_from(0.29792074959875355558e-1_f64) * t34278;
    let t34281 = t1415 * t8330 * t7030;
    let t34282 = F::cast_from(0.29792074959875355558e-1_f64) * t34281;
    let t34284 = t7025 * t2365 * t25740;
    let t34285 = F::cast_from(0.14896037479937677779e-1_f64) * t34284;
    let t34286 = t544 * t26126;
    let t34288 = F::cast_from(0.50050685932590597338e1_f64) * t34286 * t21139;
    let t34291 = F::cast_from(0.85801175884441024006e1_f64) * t18067 * t6964 * t10513;
    (t34279, t34282, t34285, t34288, t34291)
}
