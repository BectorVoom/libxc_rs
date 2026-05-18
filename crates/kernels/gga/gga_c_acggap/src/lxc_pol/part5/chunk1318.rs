//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1318/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1318<F: Float>(t11772: F, t11775: F, t11778: F, t11780: F, t11792: F, t11825: F, t19995: F, t19996: F, t19997: F, t19998: F, t6592: F, t694: F, t839: F) -> F {
    let t24578 = F::new(3.0) * t6592 * t694 * t839 - t11772 - t11775 + t11778 - t11780 + t11792 + t11825 - t19995 - t19996 + t19997 - t19998;
    t24578
}
