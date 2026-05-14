//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 946/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk946<F: Float>(t12527: F, t3416: F, t4933: F, t1318: F, t3899: F, t5316: F, t2158: F, t9752: F, t4646: F, t518: F, t525: F, t11636: F, t12508: F, t12509: F, t12514: F, t12520: F, t12523: F, t12524: F, t12525: F, t225: F, t231: F) -> (F, F, F, F, F, F) {
    let t12528 = 8.0 / 15.0 * t12527;
    let t12529 = t3416 * t4933;
    let t12530 = 16.0 / 15.0 * t12529;
    let t12532 = t1318 * t3899 * t5316;
    let t12533 = 16.0 / 15.0 * t12532;
    let t12535 = 4.0 / 5.0 * t9752 * t2158;
    let t12536 = t4646 * t518;
    let t12538 = 4.0 / 15.0 * t12536 * t525;
    let t12539 = t12508 + 4.0 / 3.0 * t12509 + 4.0 / 3.0 * t11636 * t225 * t231 + 4.0 * t12514 - t12520 + t12523 + t12524 - t12525 - t12528 - t12530 - t12533 - t12535 + t12538;
    (t12528, t12530, t12533, t12535, t12538, t12539)
}
