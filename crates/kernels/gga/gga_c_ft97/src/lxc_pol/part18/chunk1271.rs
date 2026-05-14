//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1271/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1271<F: Float>(t1360: F, t7954: F, t165: F, t7763: F, t23405: F, t27417: F, t26791: F, t378: F, t11437: F, t11982: F, t13043: F, t1642: F, t23413: F, t24080: F, t24081: F, t24082: F, t24083: F, t26533: F, t26800: F, t26801: F, t26817: F, t27420: F, t27422: F, t27426: F, t27427: F, t27428: F, t379: F, t5772: F, t5843: F, t94148: F, t94230: F, t95009: F) -> (F,) {
    let t104150 = t7954 * t1360;
    let t104151 = t165 * t7763;
    let t104157 = 2.0 / 27.0 * t23405 * t27417;
    let t104161 = t378 * t26791;
    let t104173 = 2.0 / 9.0 * t5772 * t378 * t5843 * t27422 - 2.0 / 27.0 * t5772 * t1642 * t5843 * t27428 + 2.0 / 9.0 * t23413 * t26801 + 2.0 / 9.0 * t5772 * t94230 * t26800 + 2.0 / 9.0 * t5772 * t24080 * t26533 * t379 - t5772 * t27426 * t27427 * t11982 / 27.0 - 5.0 / 81.0 * t5772 * t104150 * t104151 * t11437 - t104157 + 2.0 / 9.0 * t26817 * t24083 + 2.0 / 9.0 * t94148 + 2.0 / 9.0 * t5772 * t104161 * t24082 - t5772 * t95009 * t24081 * t13043 / 3.0 - t5772 * t27420 * t27427 * t11437 / 3.0;
    (t104173,)
}
