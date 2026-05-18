//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1014/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1014<F: Float>(t30268: F, t8775: F, t30105: F, t8952: F, t7839: F, t8739: F, t1181: F, t22778: F, t7564: F, t8600: F, t1983: F, t30692: F, t5720: F, t7586: F) -> (F, F, F, F, F) {
    let t33982 = t30268 * t8775;
    let t33983 = F::new(0.64311027177104605458e-2) * t33982;
    let t33984 = t30105 * t8952;
    let t33986 = t7839 * t8739;
    let t33987 = F::new(0.62896184579208304136e-3) * t33986;
    let t33990 = t7564 * t1181 * t8600 * t22778;
    let t33994 = t30692 * t7586 * t1983 * t5720;
    (t33983, t33984, t33987, t33990, t33994)
}
