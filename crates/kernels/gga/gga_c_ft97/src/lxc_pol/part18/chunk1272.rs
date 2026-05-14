//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1272/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1272<F: Float>(t1984: F, t26768: F, t23405: F, t26823: F, t165: F, t6584: F, t94329: F, t1349: F, t1360: F, t1643: F, t1651: F, t1969: F, t23925: F, t24080: F, t24081: F, t24143: F, t26538: F, t26551: F, t26553: F, t26817: F, t28: F, t3000: F, t379: F, t5766: F, t5772: F, t5779: F, t6587: F, t94175: F, t94184: F, t94976: F) -> (F, F) {
    let t104175 = t1984 * t26768;
    let t104204 = t23405 * t26823 / 27.0;
    let t104205 = t26768 * t165;
    let t104213 = t94329 * t6584 / 27.0;
    let t104214 = 4.0 / 27.0 * t94175 - 2.0 / 3.0 * t1349 * t28 * t104175 * t5779 - t1349 * t3000 * t1360 * t24081 / 9.0 - 2.0 / 3.0 * t1349 * t28 * t23925 * t26551 + 2.0 / 9.0 * t5772 * t24080 * t26538 * t379 + t5772 * t24080 * t6587 * t1651 / 9.0 + 2.0 / 27.0 * t5772 * t94976 * t6587 * t1643 - 2.0 / 3.0 * t5766 * t26553 - t94184 / 18.0 + t104204 - t5772 * t1969 * t104205 * t379 / 9.0 + t26817 * t24143 / 9.0 + t104213;
    (t104175, t104214)
}
