//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1253/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1253(t56677: f64, t7341: f64, t837: f64, t845: f64, t13796: f64, t14091: f64, t4919: f64, t2472: f64, t2476: f64, t25183: f64, t55901: f64, t2633: f64, t55906: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56681 = 0.1403573615389248977e2_f64 * t845 * t7341 * t56677 * t837;
    let t56686 = 0.62336721237753107879e3_f64 * t845 * t13796 * t14091;
    let t56689 = t4919 * t4919;
    let t56693 = 0.51947267698127589897e2_f64 * t845 * t2472 * t56689 * t2476;
    let t56700 = t25183 * t55901;
    let t56704 = t2633 * t55906;
    (t56681, t56686, t56689, t56693, t56700, t56704)
}
