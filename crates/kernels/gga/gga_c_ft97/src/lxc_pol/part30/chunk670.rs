//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 670/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk670<F: Float>(t17807: F, t27566: F, t27717: F, t33351: F, t33359: F, t33362: F, t33366: F, t33368: F, t33372: F, t33375: F, t33379: F, t33380: F, t33383: F, t33385: F, t33388: F, t33390: F, t33394: F, t3762: F, t6057: F, t690: F, t710: F, t7447: F) -> (F,) {
    let t33398 = -0.23254900946437792e-1 * t33351 * t690 - 2.0 * t7447 * t710 + 0.10338048737805743097e-3 * t27566 * t33359 + 0.88910709717637694816e-2 * t27717 * t33362 + 0.89080607335887169333e-3 * t33366 * t33368 + 0.15322466011111111111e0 * t33372 * t33375 - t33379 - 0.25537443351851851852e-1 * t33380 * t6057 - 0.25845121844514357744e-4 * t33383 * t33385 - 0.22227677429409423704e-2 * t33388 * t33390 - 0.22979081259345929704e-6 * t17807 * t33394 * t3762;
    (t33398,)
}
