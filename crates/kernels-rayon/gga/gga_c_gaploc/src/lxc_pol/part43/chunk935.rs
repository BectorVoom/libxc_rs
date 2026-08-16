//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 935/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk935(t1445: f64, t2087: f64, t43240: f64, t13161: f64, t5782: f64, t13149: f64, t2464: f64, t825: f64, t24968: f64, t9958: f64, t43598: f64, t7572: f64, t7573: f64) -> (f64, f64, f64, f64, f64) {
    let t44038 = 0.62115540045351614476e2_f64 * t2087 * t1445 * t43240;
    let t44040 = 0.62115540045351614476e2_f64 * t5782 * t13161;
    let t44045 = t825 * t2464 * t13149;
    let t44046 = 0.63904876589867916128e-1_f64 * t44045;
    let t44051 = 0.42900587942220512003e1_f64 * t24968 * t9958;
    let t44057 = 0.62115540045351614476e2_f64 * t7572 * t7573 * t43598;
    (t44038, t44040, t44046, t44051, t44057)
}
