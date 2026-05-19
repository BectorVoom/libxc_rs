//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1219/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1219<F: Float>(t28617: F, t38318: F, t28635: F, t28637: F, t22403: F, t22625: F, t22627: F, t22636: F, t28610: F, t28626: F, t28628: F, t4733: F) -> (F, F, F, F, F, F) {
    let t56039 = F::cast_from(0.2077890707925103596e3_f64) * t28617;
    let t56040 = F::cast_from(0.35089340384731224426e1_f64) * t38318;
    let t56043 = F::new(96.0) * t28635;
    let t56044 = F::new(576.0) * t28637;
    let t56045 = t22625 + t22627 - F::new(14.0) * t28610 - t56039 - t56040 - F::new(1820.0) / F::new(27.0) * t28626 - F::new(14.0) * t28628 - t56043 - t56044 - t22403 - t22636;
    let t56047 = t4733 * t4733;
    (t56039, t56040, t56043, t56044, t56045, t56047)
}
