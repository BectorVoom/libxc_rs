//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 933/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk933<F: Float>(t1834: F, t213: F, t225: F, t214: F, t5318: F, t111: F, t26966: F, t26722: F, t26708: F, t1509: F, t7084: F, t2047: F, t4233: F) -> (F, F, F, F, F, F, F) {
    let t90566 = t213 * t1834 * t225;
    let t90739 = t214 * t5318;
    let t92090 = t26966 * t111;
    let t92386 = t26722 * t225;
    let t92439 = t26708 * t225;
    let t92552 = t7084 * t1509;
    let t92745 = t2047 * t4233;
    (t90566, t90739, t92090, t92386, t92439, t92552, t92745)
}
