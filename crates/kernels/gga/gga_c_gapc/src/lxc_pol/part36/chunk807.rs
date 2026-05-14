//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 807/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk807<F: Float>(t10172: F, t10174: F, t10176: F, t10178: F, t10183: F, t10185: F, t10187: F, t10189: F, t10192: F, t10195: F, t10198: F, t10201: F, t10205: F, t10208: F, t10213: F, t10217: F, t10220: F, t10223: F, t10227: F, t10232: F, t10234: F, t10238: F) -> (F, F) {
    let t11085 = 0.11273261948179879581e-2 * t10172 - 0.18788769913633132635e-2 * t10174 - 0.18788769913633132635e-2 * t10176 + 0.56366309740899397906e-3 * t10178 + 0.16414765573575218917e-4 * t10183 - 0.41752822030295850301e-3 * t10185 + 0.1487444284829289667e-3 * t10187 - 0.1487444284829289667e-3 * t10189 + 0.23485962392041415794e-4 * t10192 + 0.23485962392041415794e-4 * t10195 + 0.11742981196020707897e-4 * t10198;
    let t11097 = -0.1487444284829289667e-3 * t10201 + 0.23485962392041415794e-4 * t10205 + 0.11742981196020707897e-4 * t10208 + 0.11273261948179879581e-2 * t10213 - 0.66812865812879419652e-4 * t10217 - 0.11742981196020707897e-4 * t10220 - 0.685007236434541294e-5 * t10223 - 0.685007236434541294e-5 * t10227 - 0.40598095546020480691e-6 * t10232 - 0.23485962392041415794e-4 * t10234 - 0.11742981196020707897e-4 * t10238;
    (t11085, t11097)
}
