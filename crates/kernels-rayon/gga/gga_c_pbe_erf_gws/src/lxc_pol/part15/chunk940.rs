//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 940/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk940(t1480: f64, t8309: f64, t1076: f64, t169: f64, t301: f64, t366: f64, t1500: f64, t2858: f64, t2035: f64, t2986: f64, t2990: f64, t5598: f64, t5603: f64, t5608: f64, t5612: f64, t5617: f64, t5633: f64, t5650: f64, t5666: f64, t5670: f64, t5680: f64, t8293: f64, t8296: f64, t8302: f64, t8305: f64) -> f64 {
    let t8310 = t8309 * t1480;
    let t8314 = t169 * t366 * t1076 * t301;
    let t8318 = t1500 * t2858;
    let t8323 = -3.0_f64 * t5650 * t8293 - 0.53218817823353818195e-1_f64 * t8296 - 0.11974234010254609094e-1_f64 * t5608 - 0.23948468020509218188e-1_f64 * t5612 - t5617 + 6.0_f64 * t2986 * t5680 + 6.0_f64 * t2035 * t8302 + 6.0_f64 * t8305 * t5603 - 0.18218576931715098443e-4_f64 * t8310 + 0.19816831758676854261e0_f64 * t8314 + 3.0_f64 * t5598 * t2990 + t5633 + 3.0_f64 * t2035 * t8318 - 0.54045904796391420712e-1_f64 * t5666 + 0.27119625416694458076e-2_f64 * t5670;
    t8323
}
