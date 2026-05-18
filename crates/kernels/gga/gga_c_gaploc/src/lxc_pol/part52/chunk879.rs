//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 879/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk879<F: Float>(t43907: F, t36506: F, t959: F, t11845: F, t2628: F, t13625: F, t2684: F, t7354: F, t13626: F, t2013: F, t11724: F, t2464: F, t2465: F, t825: F) -> (F, F, F, F, F, F) {
    let t45711 = F::new(0.3575048995185042667e0) * t43907;
    let t45712 = t36506 * t959;
    let t45713 = F::new(0.14896037479937677779e-1) * t45712;
    let t45716 = t11845 * t2628;
    let t45717 = F::new(0.29792074959875355558e-1) * t45716;
    let t45723 = t2684 * t7354 * t13625;
    let t45725 = t2013 * t13626;
    let t45729 = t825 * t2464 * t2465 * t11724;
    (t45711, t45713, t45717, t45723, t45725, t45729)
}
