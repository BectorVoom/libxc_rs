//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 913/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk913(t10201: f64, t159: f64, t285: f64, t3379: f64, t545: f64, t281: f64, t100: f64, t481: f64, t10186: f64, t10189: f64, t10194: f64, t10197: f64, t1501: f64, t2035: f64, t2922: f64, t2986: f64, t2990: f64, t3620: f64, t3645: f64, t3686: f64, t5612: f64, t5617: f64, t5633: f64, t8296: f64, t8310: f64, t8314: f64, t8341: f64, t988: f64) -> f64 {
    let t10203 = t10201 * t159 * t285;
    let t10207 = t3379 * t545 * t285;
    let t10208 = t281 * t10207;
    let t10214 = t481 * t100;
    let t10217 = 3.0_f64 * t2035 * t10186 + 3.0_f64 * t2986 * t10189 + 6.0_f64 * t8341 * t2990 - t988 * t10194 - t988 * t10197 + t3686 * t1501 - 0.10643763564670763639e0_f64 * t8296 - 0.11974234010254609094e-1_f64 * t281 * t10203 - 0.11974234010254609094e-1_f64 * t10208 - 0.11974234010254609094e-1_f64 * t5612 - t5617 - 0.36437153863430196886e-4_f64 * t8310 + 0.39633663517353708521e0_f64 * t8314 + t5633 - t2922 * t3620 + 6.0_f64 * t10214 * t3645;
    t10217
}
