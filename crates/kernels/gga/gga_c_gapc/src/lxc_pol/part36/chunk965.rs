//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 965/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk965<F: Float>(t11483: F, t15843: F, t2597: F, t2675: F, t189: F, t615: F, t11749: F, t933: F, t11790: F, t3367: F, t6188: F, t11794: F, t7927: F, t9554: F, t126: F, t671: F) -> (F, F, F, F, F, F) {
    let t33641 = t2675 * t11483 * t2597 * t15843;
    let t33643 = t189 * t615;
    let t33645 = t933 * t33643 * t11749;
    let t33648 = t11790 * t3367 * t6188;
    let t33653 = t11794 * t7927 * t9554;
    let t33655 = t126 * t671;
    (t33641, t33643, t33645, t33648, t33653, t33655)
}
