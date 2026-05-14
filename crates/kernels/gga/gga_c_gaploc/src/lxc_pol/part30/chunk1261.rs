//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1261/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1261<F: Float>(t2413: F, t26127: F, t34239: F, t6717: F, t6914: F, t10241: F, t1359: F, t544: F, t32745: F, t488: F, t31747: F, t493: F, t4391: F, t549: F, t7893: F, t10430: F, t2487: F, t6985: F) -> (F, F, F, F, F, F) {
    let t35211 = 0.21450293971110256002e1 * t26127 * t2413;
    let t35214 = 0.12423108009070322895e3 * t6914 * t6717 * t34239;
    let t35215 = t1359 * t10241;
    let t35216 = t544 * t35215;
    let t35219 = 0.79445533226334281486e-1 * t35216 * t32745 * t488;
    let t35220 = t493 * t31747;
    let t35225 = t4391 * t549 * t7893;
    let t35226 = 0.11916829983950142223e0 * t35225;
    let t35228 = t2487 * t6985 * t10430;
    (t35211, t35214, t35219, t35220, t35226, t35228)
}
