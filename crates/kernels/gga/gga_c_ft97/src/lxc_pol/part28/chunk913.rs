//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 913/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk913<F: Float>(t32186: F, t52: F, t938: F, t3099: F, t420: F, t71: F, t51: F, t5544: F, t58: F, t929: F, t173: F, t34433: F, t1301: F, t32259: F, t136720: F, t136843: F, t136968: F, t1608: F, t2035: F, t22736: F, t22819: F, t25779: F, t25802: F, t32133: F, t32228: F, t34435: F, t5551: F, t58585: F, t6441: F, t7195: F, t7318: F, t7857: F, t7867: F, t92278: F, t92377: F) -> (F, F, F, F) {
    let t145303 = t52 * t32186 * t938;
    let t145312 = t420 * t71 * t3099;
    let t145322 = t5544 * t51 * t58 * t929;
    let t145335 = t173 * t34433;
    let t145337 = t32259 * t1301 * t145335;
    let t145339 = -0.19762785756235085044e-4 * t7867 * t2035 * t7318 * t3099 + 0.11854761295685025975e-1 * t32228 * t145303 - 0.90845139567911167717e-8 * t1608 * t92377 * t5551 * t136968 * t6441 - 0.68116566383613497688e-3 * t22819 * t7195 * t145312 + 0.45958162518691859408e-7 * t22736 * t32133 * t25802 - 0.60102574844279699039e-6 * t7857 * t58585 * t145322 + 0.22979081259345929704e-6 * t92278 * t32133 * t25779 + 0.15322466011111111111e0 * t32259 * t1301 * t145312 + 0.15322466011111111111e0 * t136843 * t34435 + 0.15322466011111111111e0 * t136720 * t34435 + 0.51074886703703703703e-1 * t145337;
    (t145303, t145322, t145335, t145339)
}
