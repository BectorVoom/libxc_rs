//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 941/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk941<F: Float>(t1480: F, t8309: F, t1076: F, t169: F, t301: F, t366: F, t1500: F, t2858: F, t2035: F, t2986: F, t2990: F, t5598: F, t5603: F, t5608: F, t5612: F, t5617: F, t5633: F, t5650: F, t5666: F, t5670: F, t5680: F, t8293: F, t8296: F, t8302: F, t8305: F) -> F {
    let t8310 = t8309 * t1480;
    let t8314 = t169 * t366 * t1076 * t301;
    let t8318 = t1500 * t2858;
    let t8323 = -F::new(3.0) * t5650 * t8293 - F::new(0.53218817823353818195e-1) * t8296 - F::new(0.11974234010254609094e-1) * t5608 - F::new(0.23948468020509218188e-1) * t5612 - t5617 + F::new(6.0) * t2986 * t5680 + F::new(6.0) * t2035 * t8302 + F::new(6.0) * t8305 * t5603 - F::new(0.18218576931715098443e-4) * t8310 + F::new(0.19816831758676854261e0) * t8314 + F::new(3.0) * t5598 * t2990 + t5633 + F::new(3.0) * t2035 * t8318 - F::new(0.54045904796391420712e-1) * t5666 + F::new(0.27119625416694458076e-2) * t5670;
    t8323
}
