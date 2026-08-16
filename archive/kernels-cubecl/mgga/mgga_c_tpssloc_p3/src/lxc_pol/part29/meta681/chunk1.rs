//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2294/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2294<F: Float>(t7368: F, t94490: F, t15359: F, t15661: F, t1755: F, t2148: F, t24660: F, t24807: F, t24815: F, t24830: F, t27507: F, t3516: F, t4930: F, t7283: F, t7381: F, t7999: F, t85820: F, t85963: F, t86037: F, t94874: F, t94875: F, t94881: F, t94885: F, t94889: F, t94891: F) -> F {
    let t94901 = F::cast_from(0.14621636149762012769e-1_f64) * t94490 * t7368;
    let t94902 = -F::cast_from(0.10966227112321509577e-1_f64) * t86037 * t24660 * t1755 * t24815 * t15661 + F::cast_from(0.82246703342411321825e-2_f64) * t85963 * t94874 * t94875 * t3516 + F::cast_from(0.54831135561607547884e-2_f64) * t85820 * t94881 - t94885 - F::cast_from(0.21932454224643019153e-1_f64) * t27507 * t24807 + t94889 + t94891 - F::cast_from(0.21932454224643019153e-1_f64) * t7999 * t24830 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t15359 * t2148 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t4930 * t7381 + t94901;
    t94902
}
