//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1585/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1585(t1892: f64, t6861: f64, t6843: f64, t1385: f64, t22964: f64, t5741: f64, t75251: f64, t2782: f64, t4086: f64, t543: f64, t86455: f64, t14192: f64, t86445: f64, t9994: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86470 = t1892 * t6861;
    let t86506 = t1892 * t6843;
    let t86552 = t1385 * t22964;
    let t86563 = t75251 * t5741;
    let t86575 = t2782 * t4086 * t86455 * t543;
    let t86582 = t2782 * t4086 * t86470 * t543;
    let t86586 = t2782 * t14192 * t86445 * t9994;
    (t86470, t86506, t86552, t86563, t86575, t86582, t86586)
}
