//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1219/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1219<F: Float>(t10694: F, t1841: F, t10698: F, t29439: F, t5398: F, t7064: F, t8878: F, t10629: F, t5524: F, t1897: F, t27661: F, t954: F) -> (F, F, F, F, F) {
    let t32668 = t1841 * t10694;
    let t32669 = F::new(0.17090058289204942853e-2) * t32668;
    let t32670 = t29439 * t10698;
    let t32671 = F::new(0.19226315575355560709e-2) * t32670;
    let t32673 = t7064 * t8878 * t5398;
    let t32674 = F::new(0.1922631557535556071e-2) * t32673;
    let t32676 = F::new(0.17090058289204942851e-2) * t5524 * t10629;
    let t32679 = F::new(0.15381052460284448567e-1) * t1897 * t954 * t27661;
    (t32669, t32671, t32674, t32676, t32679)
}
