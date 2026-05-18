//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 927/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk927<F: Float>(t43914: F, t43917: F, t13625: F, t825: F, t826: F, t13632: F, t7416: F, t10914: F, t10915: F, t45369: F, t45316: F, t7584: F, t7585: F) -> (F, F, F, F, F, F) {
    let t45736 = F::new(0.59584149919750711116e-1) * t43914;
    let t45737 = F::new(0.59584149919750711116e-1) * t43917;
    let t45741 = t825 * t826 * t13625;
    let t45743 = t7416 * t13632;
    let t45744 = F::new(0.19171462976960374838e0) * t45743;
    let t45747 = F::new(0.21450293971110256001e2) * t10914 * t10915 * t45369;
    let t45753 = F::new(0.43710935587469654631e2) * t7584 * t7585 * t45316;
    (t45736, t45737, t45741, t45744, t45747, t45753)
}
