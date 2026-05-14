//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 833/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk833<F: Float>(t10201: F, t159: F, t285: F, t3379: F, t545: F, t281: F, t100: F, t481: F, t10186: F, t10189: F, t10194: F, t10197: F, t1501: F, t2035: F, t2922: F, t2986: F, t2990: F, t3620: F, t3645: F, t3686: F, t5612: F, t5617: F, t5633: F, t8296: F, t8310: F, t8314: F, t8341: F, t988: F) -> (F,) {
    let t10203 = t10201 * t159 * t285;
    let t10207 = t3379 * t545 * t285;
    let t10208 = t281 * t10207;
    let t10214 = t481 * t100;
    let t10217 = 3.0 * t2035 * t10186 + 3.0 * t2986 * t10189 + 6.0 * t8341 * t2990 - t988 * t10194 - t988 * t10197 + t3686 * t1501 - 0.10643763564670763639e0 * t8296 - 0.11974234010254609094e-1 * t281 * t10203 - 0.11974234010254609094e-1 * t10208 - 0.11974234010254609094e-1 * t5612 - t5617 - 0.36437153863430196886e-4 * t8310 + 0.39633663517353708521e0 * t8314 + t5633 - t2922 * t3620 + 6.0 * t10214 * t3645;
    (t10217,)
}
