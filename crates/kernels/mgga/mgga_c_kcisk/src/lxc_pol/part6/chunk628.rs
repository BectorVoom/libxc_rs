//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 628/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk628<F: Float>(t1224: F, t1697: F, t8518: F, t4835: F, t7076: F, t8684: F, t8687: F, t2417: F, t1725: F, t2408: F, t4864: F, t4868: F) -> (F, F, F, F, F, F, F) {
    let t8690 = t1224 * t1697 * t8518;
    let t8692 = t4835 + F::new(0.11872222222222222222e-1) * t7076 - F::new(0.11872222222222222222e-1) * t8684 + F::new(0.35616666666666666666e-1) * t8687 - F::new(0.17808333333333333333e-1) * t8690;
    let t8697 = t2417 * t2417;
    let t8698 = t8697 * t1725;
    let t8701 = t2408 * t2408;
    let t8702 = t4864 * t8701;
    let t8708 = t4868 + F::new(2.0) / F::new(9.0) * t7076 - F::new(2.0) / F::new(9.0) * t8684 + F::new(2.0) / F::new(3.0) * t8687 - t8690 / F::new(3.0);
    (t8690, t8692, t8697, t8698, t8701, t8702, t8708)
}
