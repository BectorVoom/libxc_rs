//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 945/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk945<F: Float>(t2630: F, t9860: F, t2516: F, t676: F, t3869: F, t2496: F, t9524: F, t9542: F, t9577: F, t9579: F, t9581: F, t9588: F, t9598: F, t9854: F, t9857: F, t9859: F) -> (F, F, F, F, F, F) {
    let t9861 = t9860 * t2630;
    let t9862 = F::cast_from(0.32530743900905219526e-1_f64) * t9861;
    let t9863 = t676 * t2516;
    let t9865 = F::cast_from(0.16265371950452609763e-1_f64) * t3869 * t9863;
    let t9866 = t676 * t2496;
    let t9868 = F::cast_from(0.48159733137676571078e0_f64) * t3869 * t9866;
    let t9869 = -t9577 + t9579 - t9581 - t9588 - t9524 + t9598 + t9542 + t9854 - t9857 - t9859 + t9862 + t9865 + t9868;
    (t9862, t9863, t9865, t9866, t9868, t9869)
}
