//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1047/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1047<F: Float>(t1301: F, t145335: F, t32259: F, t136720: F, t136843: F, t136968: F, t145303: F, t145312: F, t145322: F, t1608: F, t2035: F, t22736: F, t22819: F, t25779: F, t25802: F, t3099: F, t32133: F, t32228: F, t34435: F, t5551: F, t58585: F, t6441: F, t7195: F, t7318: F, t7857: F, t7867: F, t92278: F, t92377: F) -> F {
    let t145337 = t32259 * t1301 * t145335;
    let t145339 = -F::new(0.19762785756235085044e-4) * t7867 * t2035 * t7318 * t3099 + F::new(0.11854761295685025975e-1) * t32228 * t145303 - F::new(0.90845139567911167717e-8) * t1608 * t92377 * t5551 * t136968 * t6441 - F::new(0.68116566383613497688e-3) * t22819 * t7195 * t145312 + F::new(0.45958162518691859408e-7) * t22736 * t32133 * t25802 - F::new(0.60102574844279699039e-6) * t7857 * t58585 * t145322 + F::new(0.22979081259345929704e-6) * t92278 * t32133 * t25779 + F::new(0.15322466011111111111e0) * t32259 * t1301 * t145312 + F::new(0.15322466011111111111e0) * t136843 * t34435 + F::new(0.15322466011111111111e0) * t136720 * t34435 + F::new(0.51074886703703703703e-1) * t145337;
    t145339
}
