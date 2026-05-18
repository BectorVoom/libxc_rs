//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 608/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk608<F: Float>(t3536: F, t590: F, t3516: F, t4130: F, t10507: F, t10509: F, t10538: F, t10541: F, t10544: F, t10550: F, t10598: F, t10610: F, t10616: F, t10619: F, t1441: F, t4781: F) -> (F, F) {
    let t11536 = t3536 * t590;
    let t11549 = t4130 * t3516;
    let t11550 = t11549 * t590;
    let t11553 = F::new(0.1022478025437886658e1) * t1441 * t11536 + F::new(0.59584149919750711116e-1) * t10507 + F::new(0.59584149919750711116e-1) * t10509 + F::new(0.11916829983950142223e0) * t10538 + F::new(0.11916829983950142223e0) * t10541 + F::new(0.1022478025437886658e1) * t10544 - F::new(0.11916829983950142223e0) * t10550 + F::new(0.38342925953920749677e1) * t10598 - F::new(0.23005755572352449806e1) * t10610 - F::new(0.17875244975925213335e0) * t10616 + F::new(0.59584149919750711116e-1) * t10619 + F::new(0.15337170381568299871e1) * t4781 * t11550;
    (t11549, t11553)
}
