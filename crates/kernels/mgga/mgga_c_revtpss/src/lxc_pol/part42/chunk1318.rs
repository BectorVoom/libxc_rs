//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1318/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1318<F: Float>(t545: F, t6888: F, t869: F, t689: F, t22005: F, t4003: F, t5744: F, t2782: F, t21981: F, t4086: F, t543: F, t22009: F, t72: F, t1432: F, t686: F, t10049: F, t10117: F, t10126: F, t10129: F, t10137: F, t10143: F, t1399: F, t14252: F, t1437: F, t22253: F, t5659: F, t5735: F, t5755: F, t6862: F, t820: F) -> (F,) {
    let t22351 = t545 * t6888;
    let t22352 = t869 * t22351;
    let t22353 = t689 * t22352;
    let t22361 = t5744 * t22005 * t4003;
    let t22362 = t2782 * t22361;
    let t22365 = t4086 * t21981 * t543;
    let t22366 = t2782 * t22365;
    let t22369 = t4086 * t22009 * t543;
    let t22370 = t2782 * t22369;
    let t22373 = t4086 * t22005 * t543;
    let t22374 = t2782 * t22373;
    let t22379 = t6888 * t72;
    let t22381 = t1432 * t22379 * t686;
    let t22384 = -0.65854491829355115987e0 * t5755 * t22009 * t1399 - 0.13170898365871023197e1 * t5755 * t5735 * t5659 - t10117 - 0.54878743191129263322e-2 * t22353 + 0.13170898365871023197e1 * t820 * t10049 * t6862 - t10126 - t10129 - 0.26019841438354088051e-1 * t14252 + 0.13009920719177044025e-1 * t10137 - 0.10975748638225852664e-1 * t22362 + 0.10975748638225852664e-1 * t22366 + 0.54878743191129263322e-2 * t22370 + 0.54878743191129263322e-2 * t22374 - 0.65854491829355115987e0 * t820 * t1437 * t22253 + 0.9757440539382783019e-2 * t22381 - 0.11565819519348392139e-2 * t10143;
    (t22384,)
}
