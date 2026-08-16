//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 995/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk995(t12312: f64, t12326: f64, t209: f64, t3855: f64, t575: f64, t687: f64, t10526: f64, t1112: f64, t10529: f64, t3483: f64, t3480: f64, t3537: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12327 = t12312 + t12326;
    let t12328 = t12327 * t209;
    let t12329 = t3855 * t575;
    let t12330 = t12329 * t687;
    let t12331 = t10526 * t1112;
    let t12332 = 2.0_f64 * t12331;
    let t12333 = t10529 * t3483;
    let t12334 = 4.0_f64 * t12333;
    let t12335 = t3480 * t3537;
    (t12327, t12328, t12329, t12330, t12331, t12332, t12333, t12334, t12335)
}
