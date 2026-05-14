//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 967/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk967<F: Float>(t845: F, t875: F, t13796: F, t13859: F, t13780: F, t2410: F, t3990: F, t2195: F, t3991: F, t3989: F, t2409: F, t6143: F, t3965: F, t2307: F, t3975: F, t3972: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13860 = t845 * t875;
    let t13861 = t13796 * t13860;
    let t13862 = t13859 * t13861;
    let t13865 = t3990 * t13780 * t2410;
    let t13866 = t13859 * t13865;
    let t13869 = t3990 * t3991 * t2195;
    let t13870 = t3989 * t13869;
    let t13872 = t2409 * t6143;
    let t13873 = t3965 * t13872;
    let t13877 = t3975 * t2307;
    let t13878 = t3972 * t13877;
    (t13861, t13862, t13865, t13866, t13869, t13870, t13872, t13873, t13877, t13878)
}
