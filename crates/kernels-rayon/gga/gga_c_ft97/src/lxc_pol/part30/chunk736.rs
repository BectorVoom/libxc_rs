//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 736/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk736(t17807: f64, t27566: f64, t27717: f64, t33351: f64, t33359: f64, t33362: f64, t33366: f64, t33368: f64, t33372: f64, t33375: f64, t33379: f64, t33380: f64, t33383: f64, t33385: f64, t33388: f64, t33390: f64, t33394: f64, t3762: f64, t6057: f64, t690: f64, t710: f64, t7447: f64) -> f64 {
    let t33398 = -0.23254900946437792e-1_f64 * t33351 * t690 - 2.0_f64 * t7447 * t710 + 0.10338048737805743097e-3_f64 * t27566 * t33359 + 0.88910709717637694816e-2_f64 * t27717 * t33362 + 0.89080607335887169333e-3_f64 * t33366 * t33368 + 0.15322466011111111111e0_f64 * t33372 * t33375 - t33379 - 0.25537443351851851852e-1_f64 * t33380 * t6057 - 0.25845121844514357744e-4_f64 * t33383 * t33385 - 0.22227677429409423704e-2_f64 * t33388 * t33390 - 0.22979081259345929704e-6_f64 * t17807 * t33394 * t3762;
    t33398
}
