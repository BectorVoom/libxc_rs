//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 970/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk970<F: Float>(t7033: F, t7038: F, t7040: F, t10536: F, t4996: F, t5005: F, t5011: F, t5019: F, t5022: F, t5025: F, t5178: F, t5186: F) -> (F, F, F, F) {
    let t10592 = F::new(0.51947577317044391276e2) * t7033;
    let t10593 = F::new(0.17544670867903938621e1) * t7038;
    let t10594 = F::new(0.35089341735807877242e1) * t7040;
    let t10595 = t10536 + t4996 + t5005 - t5011 - t10592 - t10593 + t10594 + t5019 - t5022 + t5178 + t5186 + t5025;
    (t10592, t10593, t10594, t10595)
}
