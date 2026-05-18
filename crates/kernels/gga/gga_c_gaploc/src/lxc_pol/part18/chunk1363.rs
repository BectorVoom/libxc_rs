//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1363/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1363<F: Float>(t34284: F, t26126: F, t544: F, t21139: F, t10513: F, t18067: F, t6964: F, t10524: F, t10527: F, t1397: F, t10314: F, t2476: F, t580: F) -> (F, F, F, F, F) {
    let t34285 = F::new(0.14896037479937677779e-1) * t34284;
    let t34286 = t544 * t26126;
    let t34288 = F::new(0.50050685932590597338e1) * t34286 * t21139;
    let t34291 = F::new(0.85801175884441024006e1) * t18067 * t6964 * t10513;
    let t34294 = F::new(0.42900587942220512002e1) * t1397 * t10524 * t10527;
    let t34297 = F::new(0.12269736305254639897e2) * t2476 * t580 * t10314;
    (t34285, t34288, t34291, t34294, t34297)
}
