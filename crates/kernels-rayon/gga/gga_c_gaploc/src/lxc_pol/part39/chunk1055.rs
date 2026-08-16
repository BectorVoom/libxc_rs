//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1055/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1055(t13161: f64, t5782: f64, t13125: f64, t4614: f64, t813: f64, t13149: f64, t2464: f64, t825: f64, t10930: f64, t10931: f64, t43490: f64, t24968: f64, t9958: f64) -> (f64, f64, f64, f64, f64) {
    let t44040 = 0.62115540045351614476e2_f64 * t5782 * t13161;
    let t44042 = t813 * t4614 * t13125;
    let t44045 = t825 * t2464 * t13149;
    let t44046 = 0.63904876589867916128e-1_f64 * t44045;
    let t44048 = t10930 * t10931 * t43490;
    let t44051 = 0.42900587942220512003e1_f64 * t24968 * t9958;
    (t44040, t44042, t44046, t44048, t44051)
}
