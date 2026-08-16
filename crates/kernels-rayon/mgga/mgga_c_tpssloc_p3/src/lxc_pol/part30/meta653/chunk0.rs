//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2068/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2068(t25471: f64, t82431: f64, t7607: f64, t82632: f64, t25490: f64, t82514: f64, t3030: f64, t343: f64, t25483: f64, t25486: f64, t25492: f64, t23478: f64, t4547: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t89445 = 0.18277045187202515961e-2_f64 * t82431 * t25471;
    let t89449 = t82632 * t7607;
    let t89468 = t82514 * t25490;
    let t89499 = t343 * t3030;
    let t89501 = t89499 * t25483 * t25486;
    let t89505 = t89499 * t25490 * t25492;
    let t89532 = t4547 * t23478;
    (t89445, t89449, t89468, t89501, t89505, t89532)
}
