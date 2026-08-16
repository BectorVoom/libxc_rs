//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 945/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk945(t2630: f64, t9860: f64, t2516: f64, t676: f64, t3869: f64, t2496: f64, t9524: f64, t9542: f64, t9577: f64, t9579: f64, t9581: f64, t9588: f64, t9598: f64, t9854: f64, t9857: f64, t9859: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9861 = t9860 * t2630;
    let t9862 = 0.32530743900905219526e-1_f64 * t9861;
    let t9863 = t676 * t2516;
    let t9865 = 0.16265371950452609763e-1_f64 * t3869 * t9863;
    let t9866 = t676 * t2496;
    let t9868 = 0.48159733137676571078e0_f64 * t3869 * t9866;
    let t9869 = -t9577 + t9579 - t9581 - t9588 - t9524 + t9598 + t9542 + t9854 - t9857 - t9859 + t9862 + t9865 + t9868;
    (t9862, t9863, t9865, t9866, t9868, t9869)
}
