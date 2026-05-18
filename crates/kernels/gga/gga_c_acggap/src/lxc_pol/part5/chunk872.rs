//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 872/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk872<F: Float>(t656: F, t668: F, t691: F, t2617: F, t2623: F, t195: F, t2838: F, t2955: F, t2614: F, t2981: F, t951: F, t980: F) -> (F, F, F, F, F, F, F) {
    let t12664 = F::new(0.12842595503380418954e1) * t656 * t668 * t691;
    let t12665 = t2617 * t2623;
    let t12669 = F::new(0.38527786510141256862e1) * t656 * t195 * t2838;
    let t12672 = F::new(0.38025319932552508021e2) * t656 * t195 * t2955;
    let t12673 = t2617 * t2614;
    let t12677 = F::new(0.21687162600603479684e-1) * t656 * t195 * t2981;
    let t12719 = t980 * t951;
    (t12664, t12665, t12669, t12672, t12673, t12677, t12719)
}
