//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1083/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1083(t845: f64, t875: f64, t13796: f64, t13859: f64, t13780: f64, t2410: f64, t3990: f64, t2195: f64, t3991: f64, t3989: f64, t2409: f64, t6143: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13860 = t845 * t875;
    let t13861 = t13796 * t13860;
    let t13862 = t13859 * t13861;
    let t13865 = t3990 * t13780 * t2410;
    let t13866 = t13859 * t13865;
    let t13869 = t3990 * t3991 * t2195;
    let t13870 = t3989 * t13869;
    let t13872 = t2409 * t6143;
    (t13861, t13862, t13865, t13866, t13869, t13870, t13872)
}
