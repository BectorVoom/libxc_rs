//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1358/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1358(t34239: f64, t6717: f64, t6914: f64, t10241: f64, t1359: f64, t544: f64, t32745: f64, t488: f64, t4391: f64, t549: f64, t7893: f64, t10430: f64, t2487: f64, t6985: f64) -> (f64, f64, f64, f64) {
    let t35214 = 0.12423108009070322895e3_f64 * t6914 * t6717 * t34239;
    let t35215 = t1359 * t10241;
    let t35216 = t544 * t35215;
    let t35219 = 0.79445533226334281486e-1_f64 * t35216 * t32745 * t488;
    let t35225 = t4391 * t549 * t7893;
    let t35226 = 0.11916829983950142223e0_f64 * t35225;
    let t35228 = t2487 * t6985 * t10430;
    (t35214, t35219, t35226, t35228)
}
