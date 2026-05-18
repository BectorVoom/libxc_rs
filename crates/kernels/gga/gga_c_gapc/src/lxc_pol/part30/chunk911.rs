//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 911/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk911<F: Float>(t10137: F, t10140: F, t10144: F, t10148: F, t10151: F, t10154: F, t10156: F, t10160: F, t10163: F, t10165: F, t10168: F, t10172: F, t10174: F, t10176: F, t10178: F, t10183: F, t10185: F, t10187: F, t10189: F, t10192: F, t10195: F, t10198: F) -> (F, F) {
    let t11072 = -F::new(0.11273261948179879581e-2) * t10137 - F::new(0.3757753982726626527e-4) * t10140 - F::new(0.11273261948179879581e-2) * t10144 + F::new(0.11273261948179879581e-2) * t10148 + F::new(0.11273261948179879581e-2) * t10151 + F::new(0.7113065081882594864e-4) * t10154 + F::new(0.7113065081882594864e-4) * t10156 - F::new(0.16414765573575218917e-4) * t10160 - F::new(0.2227095527095980655e-5) * t10163 - F::new(0.16440173674428991056e-4) * t10165 + F::new(0.56366309740899397906e-3) * t10168;
    let t11085 = F::new(0.11273261948179879581e-2) * t10172 - F::new(0.18788769913633132635e-2) * t10174 - F::new(0.18788769913633132635e-2) * t10176 + F::new(0.56366309740899397906e-3) * t10178 + F::new(0.16414765573575218917e-4) * t10183 - F::new(0.41752822030295850301e-3) * t10185 + F::new(0.1487444284829289667e-3) * t10187 - F::new(0.1487444284829289667e-3) * t10189 + F::new(0.23485962392041415794e-4) * t10192 + F::new(0.23485962392041415794e-4) * t10195 + F::new(0.11742981196020707897e-4) * t10198;
    (t11072, t11085)
}
