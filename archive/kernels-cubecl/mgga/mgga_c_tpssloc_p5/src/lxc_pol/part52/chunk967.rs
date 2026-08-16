//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 967/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk967<F: Float>(t1920: F, t23617: F, t6680: F, t6781: F, t6805: F, t968: F, t210: F, t6795: F, t6688: F, t974: F, t381: F, t883: F) -> (F, F, F, F, F) {
    let t23619 = F::cast_from(0.18277045187202515961e-2_f64) * t1920 * t23617;
    let t23626 = t6680 * t6781;
    let t23628 = t968 * t6805;
    let t23629 = t1920 * t23628;
    let t23631 = t6795 * t210;
    let t23632 = t974 * t6688;
    let t23633 = t23631 * t23632;
    let t23634 = t381 * t883;
    (t23619, t23626, t23629, t23633, t23634)
}
