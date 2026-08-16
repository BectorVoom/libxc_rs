//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1286/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1286(t10287: f64, t577: f64, t1980: f64, t3416: f64, t1286: f64, t7689: f64, t1321: f64, t2105: f64, t3490: f64, t645: f64, t1268: f64, t4397: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41937 = t10287 * t577;
    let t42178 = t3416 * t1980;
    let t42181 = t1286 * t7689;
    let t42336 = t1321 * t2105;
    let t42719 = t3490 * t645;
    let t42962 = t4397 * t1268;
    (t41937, t42178, t42181, t42336, t42719, t42962)
}
