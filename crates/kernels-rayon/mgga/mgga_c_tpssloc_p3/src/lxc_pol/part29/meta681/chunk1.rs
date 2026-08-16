//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2294/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2294(t7368: f64, t94490: f64, t15359: f64, t15661: f64, t1755: f64, t2148: f64, t24660: f64, t24807: f64, t24815: f64, t24830: f64, t27507: f64, t3516: f64, t4930: f64, t7283: f64, t7381: f64, t7999: f64, t85820: f64, t85963: f64, t86037: f64, t94874: f64, t94875: f64, t94881: f64, t94885: f64, t94889: f64, t94891: f64) -> f64 {
    let t94901 = 0.14621636149762012769e-1_f64 * t94490 * t7368;
    let t94902 = -0.10966227112321509577e-1_f64 * t86037 * t24660 * t1755 * t24815 * t15661 + 0.82246703342411321825e-2_f64 * t85963 * t94874 * t94875 * t3516 + 0.54831135561607547884e-2_f64 * t85820 * t94881 - t94885 - 0.21932454224643019153e-1_f64 * t27507 * t24807 + t94889 + t94891 - 0.21932454224643019153e-1_f64 * t7999 * t24830 - 0.82246703342411321825e-2_f64 * t7283 * t15359 * t2148 - 0.16449340668482264365e-1_f64 * t7283 * t4930 * t7381 + t94901;
    t94902
}
